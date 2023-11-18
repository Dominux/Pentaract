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
        let id = Uuid::new_v4();

        sqlx::query(
            format!(
                "
                INSERT INTO {FILES_TABLE} (id, path, size, storage_id, is_uploaded)
                VALUES ($1, $2, $3, $4, false);
            "
            )
            .as_str(),
        )
        .bind(id)
        .bind(&in_obj.path)
        .bind(in_obj.size)
        .bind(in_obj.storage_id)
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
                WHERE storage_id = $2 {path_filter} AND is_uploaded;
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

    pub async fn delete(&self, file_id: Uuid) -> PentaractResult<()> {
        sqlx::query(format!("DELETE FROM {FILES_TABLE} WHERE id = $1").as_str())
            .bind(file_id)
            .execute(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
            .map(|_| ())
    }
}
