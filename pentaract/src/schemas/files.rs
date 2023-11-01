use axum::body::Bytes;

pub struct InFileSchema {
    pub path: String,
    pub storage_id: uuid::Uuid,
    pub file: Bytes,
}

pub const IN_FILE_SCHEMA_FIELDS_AMOUNT: usize = 3;

pub struct InFileValidationSchema {
    pub path_err: Option<String>,
    pub storage_id_err: Option<String>,
    pub file_err: Option<String>,
}
