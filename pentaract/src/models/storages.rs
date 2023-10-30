use crate::common::types::ChatId;

pub struct InStorage {
    pub name: String,
    pub user_id: uuid::Uuid,
    pub chat_id: ChatId,
}

impl InStorage {
    pub fn new(name: String, user_id: uuid::Uuid, chat_id: ChatId) -> Self {
        Self {
            name,
            user_id,
            chat_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Storage {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub chat_id: ChatId,
}

impl Storage {
    pub fn new(id: uuid::Uuid, name: String, user_id: uuid::Uuid, chat_id: ChatId) -> Self {
        Self {
            id,
            name,
            user_id,
            chat_id,
        }
    }
}
