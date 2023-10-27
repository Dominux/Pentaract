pub struct InStorageWorker {
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StorageWorker {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub token: String,
}
