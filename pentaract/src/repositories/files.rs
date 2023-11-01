use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::{PentaractError, PentaractResult};
use crate::models::files::{File, InFile};

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
                INSERT INTO {FILES_TABLE} (id, path, storage_id, is_uploaded)
                VALUES ($1, $2, $3, false);
            "
            )
            .as_str(),
        )
        .bind(id)
        .bind(in_obj.path.clone())
        .bind(in_obj.storage_id)
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::DoesNotExist("such storage".to_string())
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let storage = File::new(id, in_obj.path, in_obj.storage_id, false);
        Ok(storage)
    }

    pub async fn list_by_storage_id(&self, storage_id: Uuid) -> PentaractResult<Vec<File>> {
        sqlx::query_as(format!("SELECT * FROM {FILES_TABLE} WHERE storage_id = $1").as_str())
            .bind(storage_id)
            .fetch_all(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
    }
}
