use serde::Deserialize;

#[derive(Deserialize)]
pub struct InStorageWorkerSchema {
    pub name: String,
    pub token: String,
}
