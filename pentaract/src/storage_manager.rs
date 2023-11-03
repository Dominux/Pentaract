use sqlx::PgPool;

use crate::{
    common::channels::{ClientMessage, Method, StorageManagerListener, UploadFileData},
    config::Config,
    errors::PentaractResult,
    services::storage_manager::StorageManagerService,
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
        let result = match msg.method {
            Method::UploadFile(data) => self.upload(data).await,
        };

        let _ = msg.tx.send(result);
    }

    async fn upload(&self, data: UploadFileData) -> PentaractResult<()> {
        StorageManagerService::new(&self.db, &self.config.telegram_api_base_url)
            .upload(data)
            .await
    }
}
