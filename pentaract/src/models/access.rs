use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Access {
    pub id: Uuid,
    pub user_id: Uuid,
}
