// pub struct InUser {
//     pub username: String,
//     pub password: String,
// }

pub struct InDBUser {
    pub username: String,
    pub password_hash: String,
}

impl InDBUser {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            username,
            password_hash,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: uuid::Uuid, username: String, password_hash: String) -> Self {
        Self {
            id,
            username,
            password_hash,
        }
    }
}
