pub struct InFile {
    pub path: String,
    pub storage_id: uuid::Uuid,
}

impl InFile {
    pub fn new(path: String, storage_id: uuid::Uuid) -> Self {
        Self { path, storage_id }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct File {
    pub id: uuid::Uuid,
    pub path: String,
    pub storage_id: uuid::Uuid,
    pub is_uploaded: bool,
}

impl File {
    pub fn new(id: uuid::Uuid, path: String, storage_id: uuid::Uuid, is_uploaded: bool) -> Self {
        Self {
            id,
            path,
            storage_id,
            is_uploaded,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FSElement {
    pub path: String,
    pub is_file: bool,
}
