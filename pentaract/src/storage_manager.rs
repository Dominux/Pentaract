use sqlx::PgPool;

use crate::{
    common::channels::{ClientMessage, StorageManagerListener},
    config::Config,
};

pub struct StorageManager {
    rx: StorageManagerListener,
    db: PgPool,
    config: Config,
}

impl StorageManager {
    pub fn new(rx: StorageManagerListener, db: PgPool, config: Config) -> Self {
        Self { rx, db, config }
    }

    pub async fn run(&mut self) {
        // Start receiving messages
        while let Some(msg) = self.rx.recv().await {
            tracing::debug!("got msg");

            self.handle_msg(msg).await
        }
    }

    async fn handle_msg(&self, msg: ClientMessage) {
        // msg.
    }
}
