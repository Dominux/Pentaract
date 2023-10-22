use sqlx::{Pool, Postgres};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, config: Config) -> Self {
        Self { db, config }
    }
}
