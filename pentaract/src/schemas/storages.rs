use serde::{Deserialize, Serialize};

use crate::{common::types::ChatId, models::storages::StorageWithInfo};

#[derive(Deserialize)]
pub struct InStorageSchema {
    pub name: String,
    pub chat_id: ChatId,
}

#[derive(Serialize)]
pub struct StoragesListSchema {
    pub storages: Vec<StorageWithInfo>,
}

impl StoragesListSchema {
    pub fn new(storages: Vec<StorageWithInfo>) -> Self {
        Self { storages }
    }
}
