use sqlx::{Pool, Postgres};

use crate::{common::channels::ClientSender, config::Config};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
    pub tx: ClientSender,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, config: Config, tx: ClientSender) -> Self {
        Self { db, config, tx }
    }
}
