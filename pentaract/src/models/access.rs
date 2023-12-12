use serde::Deserialize;
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Deserialize)]
#[sqlx(type_name = "access_type", rename_all = "lowercase")]
pub enum AccessType {
    R,
    W,
    A,
}

#[derive(Deserialize)]
pub struct GrantAccess {
    pub user_email: String,
    pub storage_id: Uuid,
    pub access_type: AccessType,
}

impl GrantAccess {
    pub fn new(user_email: String, storage_id: Uuid, access_type: AccessType) -> Self {
        Self {
            user_email,
            storage_id,
            access_type,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Access {
    pub id: Uuid,
    pub user_id: Uuid,
    pub storage_id: Uuid,
    pub access_type: AccessType,
}
