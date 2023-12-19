use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InStorageWorkerSchema {
    pub name: String,
    pub token: String,
    pub storage_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct StorageWorkersStorageIDQuery {
    pub storage_id: Uuid,
}

#[derive(Serialize)]
pub struct HasStorageWorkers {
    pub has: bool,
}
