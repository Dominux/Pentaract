use serde::Deserialize;

use crate::common::types::ChatId;

#[derive(Deserialize)]
pub struct InStorageSchema {
    pub name: String,
    pub chat_id: ChatId,
}
