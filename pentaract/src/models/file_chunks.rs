use crate::common::types::Position;

#[derive(Debug, sqlx::FromRow)]
pub struct FileChunk {
    pub id: uuid::Uuid,
    pub file_id: uuid::Uuid,
    pub telegram_file_id: String,
    pub position: Position,
}

impl FileChunk {
    pub fn new(
        id: uuid::Uuid,
        file_id: uuid::Uuid,
        telegram_file_id: String,
        position: Position,
    ) -> Self {
        Self {
            id,
            file_id,
            telegram_file_id,
            position,
        }
    }
}
