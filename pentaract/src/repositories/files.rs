use std::path::Path;

use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::file_chunks::FileChunk;
use crate::models::files::{DBFSElement, FSElement, File, InFile};

pub const FILES_TABLE: &str = "files";
pub const CHUNKS_TABLE: &str = "file_chunks";

/// General repo for files and chunks since they share common logic
pub struct FilesRepository<'d> {
    db: &'d PgPool,
}

impl<'d> FilesRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create_file(&self, in_obj: InFile) -> PentaractResult<File> {
        self._create_file(in_obj, false).await
    }

    pub async fn create_folder(&self, in_obj: InFile) -> PentaractResult<File> {
        self._create_file(in_obj, true).await
    }

    async fn _create_file(&self, in_obj: InFile, is_uploaded: bool) -> PentaractResult<File> {
        let id = Uuid::new_v4();

        sqlx::query(
            format!(
                "
                INSERT INTO {FILES_TABLE} (id, path, size, storage_id, is_uploaded)
                VALUES ($1, $2, $3, $4, $5);
            "
            )
            .as_str(),
        )
        .bind(id)
        .bind(&in_obj.path)
        .bind(in_obj.size)
        .bind(in_obj.storage_id)
        .bind(is_uploaded)
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::DoesNotExist("such storage".to_string())
            }
            sqlx::Error::Database(dbe) if dbe.is_unique_violation() => {
                PentaractError::AlreadyExists("File with such name".to_string())
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let storage = File::new(id, in_obj.path, in_obj.size, in_obj.storage_id, false);
        Ok(storage)
    }

    /// Creates a file even if the given path already exists
    pub async fn create_file_anyway(&self, in_obj: InFile) -> PentaractResult<File> {
        let id = Uuid::new_v4();
        let chars_to_skip = in_obj.path.len() + 3; // if the name is `kek` then it's gonna be a len of `kek (` + 1

        // https://www.db-fiddle.com/f/i6XCvTSi5cpAVu5AAfiNqm/16
        sqlx::query_as(
            format!(
                r#"
                INSERT INTO files (path, storage_id, id, size, is_uploaded)
                WITH f AS (
                    SELECT path
                    FROM {FILES_TABLE}
                    WHERE storage_id = $2 AND path ~ ('^(' || $1 || '|' || $1 || ' \(\d+\))$')
                    ORDER BY path DESC
                )
                SELECT 
                    CASE
                        WHEN NOT EXISTS(
                            SELECT path 
                            FROM f 
                            WHERE path = $1
                        ) THEN $1
                        ELSE
                            CASE
                                WHEN COUNT(f) > 1 THEN (
                                    WITH cte AS (
                                        SELECT *
                                        FROM (
                                            SELECT SUBSTRING(f.path, {chars_to_skip}, LENGTH(f.path) - {chars_to_skip})::numeric AS i
                                            FROM f
                                            WHERE f.path != $1
                                        ) AS n
                                        WHERE i > 0
                                        ORDER BY i
                                    )
                                    SELECT $1 || ' (' || COALESCE(t.next_i, (
                                        SELECT cte.i + 1 
                                        FROM cte 
                                        ORDER BY cte.i DESC 
                                        LIMIT 1
                                    )) || ')'
                                    FROM cte
                                    FULL OUTER JOIN (
                                        SELECT prev_i + 1 AS next_i
                                        FROM (
                                            SELECT LAG(i, 1, 0) OVER() AS prev_i, i
                                            FROM cte
                                        ) t
                                        WHERE prev_i != t.i - 1
                                        LIMIT 1
                                    ) t ON cte.i = t.next_i
                                    LIMIT 1
                                )
                                WHEN COUNT(f) = 1 THEN $1 || ' (1)'
                                ELSE $1
                            END
                    END,
                    $2,
                    $3,
                    $4,
                    false
                FROM f
                RETURNING *;
            "#
            )
            .as_str(),
        )
        .bind(&in_obj.path)
        .bind(in_obj.storage_id)
        .bind(id)
        .bind(in_obj.size)
        .fetch_one(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::DoesNotExist("such storage".to_string())
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })
    }

    pub async fn create_chunks_batch(&self, chunks: Vec<FileChunk>) -> PentaractResult<()> {
        QueryBuilder::new(
            format!("INSERT INTO {CHUNKS_TABLE} (id, file_id, telegram_file_id, position)")
                .as_str(),
        )
        .push_values(chunks, |mut q, chunk| {
            q.push_bind(chunk.id)
                .push_bind(chunk.file_id)
                .push_bind(chunk.telegram_file_id)
                .push_bind(chunk.position);
        })
        .build()
        .execute(self.db)
        .await
        .map_err(|_| PentaractError::Unknown)?;

        Ok(())
    }

    /// NOTE:
    ///
    /// `prefix` must be without leading and trailing slashes
    pub async fn list_dir(
        &self,
        storage_id: Uuid,
        prefix: &str,
    ) -> PentaractResult<Vec<FSElement>> {
        let query = {
            let adding_to_position = !prefix.is_empty() as usize + 1;
            let split_position = prefix.matches("/").count() + adding_to_position;
            let split_part = format!("SPLIT_PART(path, '/', {split_position})");
            let path_filter = if prefix.is_empty() {
                ""
            } else {
                "AND path LIKE $1 || '%'"
            };

            format!(
                "
                SELECT DISTINCT {split_part} AS name, $1 || {split_part} = path AS is_file 
                FROM {FILES_TABLE} 
                WHERE storage_id = $2 {path_filter} AND is_uploaded AND {split_part} <> '';
            "
            )
        };

        let prefix = if prefix.is_empty() {
            prefix.to_string()
        } else {
            format!("{prefix}/")
        };

        let fs_layer = sqlx::query_as::<_, DBFSElement>(&query)
            .bind(&prefix)
            .bind(storage_id)
            .fetch_all(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)?;
        let fs_layer = fs_layer
            .into_iter()
            .map(|el| {
                let path = format!("{prefix}{}", el.name);
                FSElement {
                    path,
                    name: el.name,
                    is_file: el.is_file,
                }
            })
            .collect();

        Ok(fs_layer)
    }

    pub async fn get_file_by_path(&self, path: &str, storage_id: Uuid) -> PentaractResult<File> {
        sqlx::query_as(
            format!("SELECT * FROM {FILES_TABLE} WHERE storage_id = $1 AND path = $2").as_str(),
        )
        .bind(storage_id)
        .bind(path)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "file"))
    }

    pub async fn list_chunks_of_file(&self, file_id: Uuid) -> PentaractResult<Vec<FileChunk>> {
        sqlx::query_as(format!("SELECT * FROM {CHUNKS_TABLE} WHERE file_id = $1").as_str())
            .bind(file_id)
            .fetch_all(self.db)
            .await
            .map_err(|e| map_not_found(e, "file chunks"))
    }

    pub async fn set_as_uploaded(&self, file_id: Uuid) -> PentaractResult<()> {
        sqlx::query(format!("UPDATE {FILES_TABLE} SET is_uploaded = true WHERE id = $1").as_str())
            .bind(file_id)
            .execute(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
            .map(|_| ())
    }
    pub async fn delete_with_folders(&self, id: Uuid) -> PentaractResult<()> {
        sqlx::query(format!("DELETE FROM {FILES_TABLE} WHERE id = $1").as_str())
            .bind(id)
            .execute(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
            .map(|_| ())
    }

    pub async fn delete(&self, path: &str, storage_id: Uuid) -> PentaractResult<()> {
        let mut transaction = self.db.begin().await.map_err(|e| map_not_found(e, ""))?;

        // deleting file
        sqlx::query(&format!(
            "
            DELETE FROM {FILES_TABLE} 
            WHERE storage_id = $1 AND path LIKE $2 || '%';
            "
        ))
        .bind(storage_id)
        .bind(path)
        .execute(&mut *transaction)
        .await
        .map_err(|e| map_not_found(e, "file"))?;

        // creating a folder if it was the file in the folder
        if let Some(parent) = Path::new(path).parent().map(|path| path.to_str().unwrap()) {
            let new_id = Uuid::new_v4();
            let parent = format!("{parent}/");

            sqlx::query(&format!(
                "
                INSERT INTO {FILES_TABLE} (id, path, size, storage_id, is_uploaded)
                SELECT $1, $2, 0, $3, true 
                WHERE 
                    NOT EXISTS (
                        SELECT id 
                        FROM {FILES_TABLE}
                        WHERE storage_id = $3 AND path LIKE $2 || '%'
                    );
            "
            ))
            .bind(new_id)
            .bind(parent)
            .bind(storage_id)
            .execute(&mut *transaction)
            .await
            .map_err(|e| map_not_found(e, "some entity"))?;
        }

        transaction
            .commit()
            .await
            .map_err(|e| map_not_found(e, ""))?;

        Ok(())
    }
}
