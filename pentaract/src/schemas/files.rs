use axum::body::Bytes;
use uuid::Uuid;

use crate::common::types::Position;

pub struct InFileSchema {
    pub storage_id: Uuid,
    pub path: String,
    pub file: Bytes,
}

pub const IN_FILE_SCHEMA_FIELDS_AMOUNT: usize = 2;

pub struct DownloadedChunkSchema {
    pub position: Position,
    pub data: Vec<u8>,
}

impl DownloadedChunkSchema {
    pub fn new(position: Position, data: Vec<u8>) -> Self {
        Self { position, data }
    }
}
