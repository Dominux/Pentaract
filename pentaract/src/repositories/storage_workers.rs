use sqlx::PgPool;
use uuid::Uuid;

use crate::common::db::errors::map_not_found;
use crate::errors::{PentaractError, PentaractResult};
use crate::models::storage_workers::{InStorageWorker, StorageWorker, StorageWorkerTokenOnly};

const STORAGE_WORKERS_TABLE: &str = "storage_workers";
const STORAGE_WORKERS_USAGES_TABLE: &str = "storage_workers_usages";

pub struct StorageWorkersRepository<'d> {
    db: &'d PgPool,
}

impl<'d> StorageWorkersRepository<'d> {
    pub fn new(db: &'d PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, in_obj: InStorageWorker) -> PentaractResult<StorageWorker> {
        let id = Uuid::new_v4();

        sqlx::query(&format!(
            "
            INSERT INTO {STORAGE_WORKERS_TABLE} (id, name, token, user_id, storage_id)
            VALUES ($1, $2, $3, $4, $5);
        "
        ))
        .bind(id)
        .bind(in_obj.name.clone())
        .bind(in_obj.token.clone())
        .bind(in_obj.user_id)
        .bind(in_obj.storage_id)
        .execute(self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.is_unique_violation() => {
                PentaractError::StorageWorkerTokenConflict
            }
            sqlx::Error::Database(dbe) if dbe.is_foreign_key_violation() => {
                PentaractError::DoesNotExist("Such storage does not exist".to_string())
            }
            _ => {
                tracing::error!("{e}");
                PentaractError::Unknown
            }
        })?;

        let sw = StorageWorker::new(
            id,
            in_obj.name,
            in_obj.user_id,
            in_obj.token,
            in_obj.storage_id,
        );
        Ok(sw)
    }

    pub async fn storage_has_any(&self, storage_id: Uuid) -> PentaractResult<bool> {
        let has_sws: (_,) = sqlx::query_as(&format!(
            "SELECT COUNT(*) > 0 FROM {STORAGE_WORKERS_TABLE} WHERE storage_id = $1"
        ))
        .bind(storage_id)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "storage_workers"))?;

        Ok(has_sws.0)
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> PentaractResult<Vec<StorageWorker>> {
        sqlx::query_as(&format!(
            "SELECT * FROM {STORAGE_WORKERS_TABLE} WHERE user_id = $1"
        ))
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
        sqlx::query_as(&format!(
            "SELECT * FROM {STORAGE_WORKERS_TABLE} WHERE name = $1 AND user_id = $2"
        ))
        .bind(name)
        .bind(user_id)
        .fetch_one(self.db)
        .await
        .map_err(|e| map_not_found(e, "storage_worker"))
    }

    // https://www.db-fiddle.com/f/fHcCh7bRtVSxyDfPvPyDre/11
    pub async fn get_token(
        &self,
        storage_id: Uuid,
        limit: u8,
    ) -> PentaractResult<Option<StorageWorkerTokenOnly>> {
        let mut transaction = self.db.begin().await.map_err(|e| map_not_found(e, ""))?;

        // deleting old rows
        sqlx::query(&format!(
            "
            DELETE FROM {STORAGE_WORKERS_USAGES_TABLE}
            WHERE dt < NOW() - INTERVAL '1 minute';
            "
        ))
        .execute(&mut *transaction)
        .await
        .map_err(|e| map_not_found(e, "some entity"))?;

        let new_id = Uuid::new_v4();

        // trying to take a token and to register it's usage
        let token = sqlx::query_as(&format!(
            "
            WITH swu AS (
                INSERT INTO {STORAGE_WORKERS_USAGES_TABLE} (id, storage_worker_id)
                WITH sw AS (
                    SELECT sw.id AS storage_worker_id
                    FROM {STORAGE_WORKERS_TABLE} sw
                    LEFT JOIN {STORAGE_WORKERS_USAGES_TABLE} swu ON sw.id = swu.storage_worker_id
                    WHERE sw.storage_id = $1
                    GROUP BY sw.id
                    HAVING COUNT(swu.id) < $2
                    ORDER BY COUNT(swu.id)
                    LIMIT 1
                )
                SELECT $3, storage_worker_id FROM sw
                RETURNING storage_worker_id
            )
            SELECT token
            FROM swu
            JOIN {STORAGE_WORKERS_TABLE} sw ON swu.storage_worker_id = sw.id;
        "
        ))
        .bind(storage_id)
        .bind(limit as i16)
        .bind(new_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| map_not_found(e, "some entity"))?;

        transaction
            .commit()
            .await
            .map_err(|e| map_not_found(e, ""))?;

        Ok(token)
    }
}
