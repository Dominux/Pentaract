pub struct InUser {
    pub email: String,
    pub password: String
}


pub struct InDBUser {
    pub email: String,
    pub password_hash: String
}


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String
}

impl User {
    pub fn new(id: uuid::Uuid, email: String, password_hash: String) -> Self {
        Self { id, email, password_hash }
    }
}
