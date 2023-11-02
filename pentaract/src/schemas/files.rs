use axum::body::Bytes;
use uuid::Uuid;

pub struct InFileSchema {
    pub storage_id: Uuid,
    pub path: String,
    pub file: Bytes,
}

pub const IN_FILE_SCHEMA_FIELDS_AMOUNT: usize = 2;
