use serde::Deserialize;

#[derive(Deserialize)]
pub struct InUser {
    pub username: String,
    pub password: String,
}
