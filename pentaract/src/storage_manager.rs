use sqlx::PgPool;

use crate::{
    common::channels::{
        ClientData, ClientMessage, DownloadFileData, StorageManagerData, StorageManagerListener,
        StorageManagerMessage, UploadFileData,
    },
    config::Config,
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
        let result = match msg.data {
            ClientData::UploadFile(data) => self.upload(data).await,
            ClientData::DownloadFile(data) => self.download(data).await,
        };
        let msg_back = StorageManagerMessage::new(result);

        let _ = msg.tx.send(msg_back);
    }

    async fn upload(&self, data: UploadFileData) -> StorageManagerData {
        let result = StorageManagerService::new(
            &self.db,
            &self.config.telegram_api_base_url,
            self.config.telegram_rate_limit,
        )
        .upload(data)
        .await;

        StorageManagerData::UploadFile(result)
    }

    async fn download(&self, data: DownloadFileData) -> StorageManagerData {
        let result = StorageManagerService::new(
            &self.db,
            &self.config.telegram_api_base_url,
            self.config.telegram_rate_limit,
        )
        .download(data)
        .await;

        StorageManagerData::DownloadFile(result)
    }
}
