use axum::body::Bytes;
use uuid::Uuid;

use crate::common::types::Position;

pub struct InFileSchema {
    pub storage_id: Uuid,
    pub path: String,
    pub size: i64,
    pub file: Bytes,
}

impl InFileSchema {
    pub fn new(storage_id: Uuid, path: String, file: Bytes) -> Self {
        let size = file.len() as i64;
        Self {
            storage_id,
            path,
            size,
            file,
        }
    }
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
