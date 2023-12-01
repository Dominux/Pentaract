use serde::{Deserialize, Serialize};

use crate::{common::types::ChatId, models::storages::Storage};

#[derive(Deserialize)]
pub struct InStorageSchema {
    pub name: String,
    pub chat_id: ChatId,
}

#[derive(Serialize)]
pub struct StoragesListSchema {
    pub storages: Vec<Storage>,
}

impl StoragesListSchema {
    pub fn new(storages: Vec<Storage>) -> Self {
        Self { storages }
    }
}
