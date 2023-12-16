use sqlx::PgPool;
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::storages::{InStorage, Storage, StorageWithInfo};
use crate::repositories::{access::TABLE as ACCESS_TABLE, files::FILES_TABLE};

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
            format!("INSERT INTO {TABLE} (id, name, chat_id) VALUES ($1, $2, $3)").as_str(),
        )
        .bind(id)
        .bind(in_obj.name.clone())
        .bind(in_obj.chat_id)
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::UserWasRemoved
            }
            sqlx::Error::Database(dbe) if dbe.is_unique_violation() => {
                PentaractError::StorageChatIdConflict
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let storage = Storage::new(id, in_obj.name, in_obj.chat_id);
        Ok(storage)
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> PentaractResult<Vec<StorageWithInfo>> {
        sqlx::query_as(
            format!(
                "
                SELECT s.*, COUNT(f.id) AS files_amount, COALESCE(SUM(f.size), 0)::BigInt as size 
                FROM {TABLE} s
                JOIN {ACCESS_TABLE} a ON s.id = a.storage_id
                LEFT JOIN {FILES_TABLE} f ON s.id = f.storage_id
                WHERE a.user_id = $1 AND (f.path NOT LIKE '%/' OR f.path IS NULL)
                GROUP by s.id
            "
            )
            .as_str(),
        )
        .bind(user_id)
        .fetch_all(self.db)
        .await
        .map_err(|e| map_not_found(e, "storages"))
    }

    pub async fn get_by_id(&self, id: Uuid) -> PentaractResult<Storage> {
        sqlx::query_as(format!("SELECT * FROM {TABLE} WHERE id = $1").as_str())
            .bind(id)
            .fetch_one(self.db)
            .await
            .map_err(|e| map_not_found(e, "storage"))
    }

    pub async fn get_by_name_and_user_id(
        &self,
        name: &str,
        user_id: Uuid,
    ) -> PentaractResult<Storage> {
        sqlx::query_as(
            format!(
                "
                SELECT s.* 
                FROM {TABLE} s
                JOIN {ACCESS_TABLE} a ON s.id = a.storage_id
                WHERE a.user_id = $1 AND s.name = $2
            "
            )
            .as_str(),
        )
        .bind(user_id)
        .bind(name)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "storage"))
    }

    pub async fn get_by_file_id(&self, file_id: Uuid) -> PentaractResult<Storage> {
        sqlx::query_as(
            format!("SELECT s.* FROM {TABLE} s JOIN {FILES_TABLE} AS f ON f.storage_id = s.id WHERE f.id = $1").as_str(),
        )
        .bind(file_id)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "storage"))
    }

    pub async fn delete_storage(&self, storage_id: Uuid) -> PentaractResult<()> {
        sqlx::query(format!("DELETE FROM {TABLE} WHERE id = $1").as_str())
            .bind(storage_id)
            .execute(self.db)
            .await
            .map_err(|e| map_not_found(e, "storage"))?;
        Ok(())
    }
}
