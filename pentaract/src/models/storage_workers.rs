pub struct InStorageWorker {
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
}

impl InStorageWorker {
    pub fn new(name: String, user_id: uuid::Uuid, token: String) -> Self {
        Self {
            name,
            user_id,
            token,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct StorageWorker {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
}

impl StorageWorker {
    pub fn new(id: uuid::Uuid, name: String, user_id: uuid::Uuid, token: String) -> Self {
        Self {
            id,
            name,
            user_id,
            token,
        }
    }
}
