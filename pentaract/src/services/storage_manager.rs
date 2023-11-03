use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::{
        channels::UploadFileData,
        telegram_api::{bot_api::TelegramBotApi, schemas::InUploadSchema},
    },
    errors::{PentaractError, PentaractResult},
    models::file_chunks::FileChunk,
    repositories::{
        files::FilesRepository, storage_workers::StorageWorkersRepository,
        storages::StoragesRepository,
    },
};

pub struct StorageManagerService<'d> {
    storages_repo: StoragesRepository<'d>,
    sw_repo: StorageWorkersRepository<'d>,
    files_repo: FilesRepository<'d>,
    telegram_baseurl: &'d str,
    chunk_size: usize,
}

impl<'d> StorageManagerService<'d> {
    pub fn new(db: &'d PgPool, telegram_baseurl: &'d str) -> Self {
        let files_repo = FilesRepository::new(db);
        let storages_repo = StoragesRepository::new(db);
        let sw_repo = StorageWorkersRepository::new(db);
        let chunk_size = 20 * 1024 * 1024;
        Self {
            storages_repo,
            sw_repo,
            files_repo,
            chunk_size,
            telegram_baseurl,
        }
    }

    pub async fn upload(&self, data: UploadFileData) -> PentaractResult<()> {
        // 1. getting storage
        let storage = self.storages_repo.get_by_file_id(data.file_id).await?;

        // 2. dividing file into chunks
        let bytes_chunks = data.file_data.chunks(self.chunk_size);

        // 3. uploading by chunks
        let mut chunks = Vec::with_capacity(bytes_chunks.len());
        for (position, bytes_chunk) in bytes_chunks.enumerate() {
            let token = self.get_token(data.user_id).await?;

            let in_schema = InUploadSchema::new(bytes_chunk, storage.chat_id);
            let document = TelegramBotApi::new(self.telegram_baseurl, &token)
                .upload(&in_schema)
                .await?;

            let chunk = FileChunk::new(
                Uuid::new_v4(),
                data.file_id,
                document.file_id,
                position as i32,
            );
            chunks.push(chunk)
        }

        // 4. saving chunks to db
        self.files_repo.create_chunks_batch(chunks).await
    }

    async fn get_token(&self, user_id: Uuid) -> PentaractResult<String> {
        // TODO: add logic
        let sw_list = self.sw_repo.list_by_user_id(user_id).await?;
        let sw = sw_list.get(0).ok_or(PentaractError::NoStorageWorkers)?;
        Ok(sw.token.to_string())
    }
}
