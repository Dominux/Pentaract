use serde::Serialize;

pub struct InStorageWorker {
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub storage_id: Option<uuid::Uuid>,
}

impl InStorageWorker {
    pub fn new(
        name: String,
        user_id: uuid::Uuid,
        token: String,
        storage_id: Option<uuid::Uuid>,
    ) -> Self {
        Self {
            name,
            user_id,
            token,
            storage_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageWorker {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub storage_id: Option<uuid::Uuid>,
}

impl StorageWorker {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        user_id: uuid::Uuid,
        token: String,
        storage_id: Option<uuid::Uuid>,
    ) -> Self {
        Self {
            id,
            name,
            user_id,
            token,
            storage_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct StorageWorkerTokenOnly {
    pub token: String,
}
