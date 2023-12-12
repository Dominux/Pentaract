pub struct InDBUser {
    pub email: String,
    pub password_hash: String,
}

impl InDBUser {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            email,
            password_hash,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: uuid::Uuid, email: String, password_hash: String) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }
}
