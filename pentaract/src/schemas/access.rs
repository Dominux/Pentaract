use serde::Deserialize;
use uuid::Uuid;

use crate::models::access::AccessType;

#[derive(Deserialize)]
pub struct GrantAccess {
    pub user_email: String,
    pub access_type: AccessType,
}

impl GrantAccess {
    pub fn new(user_email: String, access_type: AccessType) -> Self {
        Self {
            user_email,
            access_type,
        }
    }
}

#[derive(Deserialize)]
pub struct RestrictAccess {
    pub user_id: Uuid,
}
