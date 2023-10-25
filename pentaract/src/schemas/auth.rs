use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginSchema {
    pub username: String,
    pub password: String,
}
