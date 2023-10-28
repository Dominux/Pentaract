use sqlx::PgPool;
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::storage_workers::{InStorageWorker, StorageWorker};

pub struct StorageWorkersRepository<'d> {
    db: &'d PgPool,
}

impl<'d> StorageWorkersRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, in_obj: InStorageWorker) -> PentaractResult<StorageWorker> {
        let id = Uuid::new_v4();

        sqlx::query_as(
            r#"
                INSERT INTO storage_workers (id, name, token, user_id)
                VALUES ($1, $2, $3, $4);
            "#,
        )
        .bind(id)
        .bind(in_obj.name)
        .bind(in_obj.token)
        .bind(in_obj.user_id)
        .fetch_one(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_unique_violation() => {
                PentaractError::StorageWorkerTokenConflict
            }
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::UserWasRemoved
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> PentaractResult<Vec<StorageWorker>> {
        sqlx::query_as("SELECT * FROM storage_workers WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
    }

    pub async fn get_by_name_and_user_id(
        &self,
        name: &str,
        user_id: Uuid,
    ) -> PentaractResult<StorageWorker> {
        sqlx::query_as("SELECT * FROM storage_workers WHERE name = $1 AND user_id = $2")
            .bind(name)
            .bind(user_id)
            .fetch_one(self.db)
            .await
            .map_err(|e| map_not_found(e, "storage_worker"))
    }
}
