use serde::Deserialize;

#[derive(Deserialize)]
pub struct InUser {
    pub email: String,
    pub password: String,
}
