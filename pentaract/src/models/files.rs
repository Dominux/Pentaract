use serde::Serialize;

pub struct InFile {
    pub path: String,
    pub size: i64,
    pub storage_id: uuid::Uuid,
}

impl InFile {
    pub fn new(path: String, size: i64, storage_id: uuid::Uuid) -> Self {
        Self {
            path,
            size,
            storage_id,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct File {
    pub id: uuid::Uuid,
    pub path: String,
    pub size: i64,
    pub storage_id: uuid::Uuid,
    pub is_uploaded: bool,
}

impl File {
    pub fn new(
        id: uuid::Uuid,
        path: String,
        size: i64,
        storage_id: uuid::Uuid,
        is_uploaded: bool,
    ) -> Self {
        Self {
            id,
            path,
            size,
            storage_id,
            is_uploaded,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct DBFSElement {
    pub name: String,
    pub size: i64,
    pub is_file: bool,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct FSElement {
    pub path: String,
    pub name: String,
    pub size: i64,
    pub is_file: bool,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SearchFSElement {
    pub path: String,
    pub is_file: bool,
}
