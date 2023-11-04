use sqlx::PgPool;
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::storages::{InStorage, Storage};
use crate::repositories::files::FILES_TABLE;

pub const TABLE: &str = "storages";

pub struct StoragesRepository<'d> {
    db: &'d PgPool,
}

impl<'d> StoragesRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, in_obj: InStorage) -> PentaractResult<Storage> {
        let id = Uuid::new_v4();

        sqlx::query(
            format!(
                "
                INSERT INTO {TABLE} (id, name, chat_id, user_id)
                VALUES ($1, $2, $3, $4);
            "
            )
            .as_str(),
        )
        .bind(id)
        .bind(in_obj.name.clone())
        .bind(in_obj.chat_id)
        .bind(in_obj.user_id)
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::UserWasRemoved
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let storage = Storage::new(id, in_obj.name, in_obj.user_id, in_obj.chat_id);
        Ok(storage)
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> PentaractResult<Vec<Storage>> {
        sqlx::query_as(format!("SELECT * FROM {TABLE} WHERE user_id = $1").as_str())
            .bind(user_id)
            .fetch_all(self.db)
            .await
            .map_err(|_| PentaractError::Unknown)
    }

    pub async fn get_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> PentaractResult<Storage> {
        sqlx::query_as(format!("SELECT * FROM {TABLE} WHERE id = $1 AND user_id = $2").as_str())
            .bind(id)
            .bind(user_id)
            .fetch_one(self.db)
            .await
            .map_err(|e| map_not_found(e, "storage_worker"))
    }

    pub async fn get_by_name_and_user_id(
        &self,
        name: &str,
        user_id: Uuid,
    ) -> PentaractResult<Storage> {
        sqlx::query_as(format!("SELECT * FROM {TABLE} WHERE name = $1 AND user_id = $2").as_str())
            .bind(name)
            .bind(user_id)
            .fetch_one(self.db)
            .await
            .map_err(|e| map_not_found(e, "storage_worker"))
    }

    pub async fn get_by_file_id(&self, file_id: Uuid) -> PentaractResult<Storage> {
        sqlx::query_as(
            format!("SELECT * FROM {TABLE} s JOIN {FILES_TABLE} AS f ON f.storage_id = s.id WHERE f.id = $1").as_str(),
        )
        .bind(file_id)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "storage_worker"))
    }
}
