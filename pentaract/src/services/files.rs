use axum::body::Bytes;
use sqlx::PgPool;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    common::{
        channels::{
            ClientData, ClientMessage, ClientSender, DownloadFileData, StorageManagerData,
            UploadFileData,
        },
        jwt_manager::AuthUser,
    },
    errors::{PentaractError, PentaractResult},
    models::files::{FSElement, File, InFile},
    repositories::files::FilesRepository,
    schemas::files::{InFileSchema, InFolderSchema},
};

pub struct FilesService<'d> {
    repo: FilesRepository<'d>,
    tx: ClientSender,
}

impl<'d> FilesService<'d> {
    pub fn new(db: &'d PgPool, tx: ClientSender) -> Self {
        let repo = FilesRepository::new(db);
        Self { repo, tx }
    }

    pub async fn create_folder(
        &self,
        in_schema: InFolderSchema,
        _user: &AuthUser,
    ) -> PentaractResult<()> {
        // 0. validation
        if !Self::validate_filepath(&in_schema.parent_path) {
            return Err(PentaractError::InvalidPath);
        }
        if in_schema.folder_name.contains(r"/") {
            return Err(PentaractError::InvalidFolderName);
        }

        // 1. constructing final values
        let path = format!("{}/{}/", in_schema.parent_path, in_schema.folder_name);
        let in_file = InFile::new(path, 0, in_schema.storage_id);

        // 2. saving to db
        self.repo.create_folder(in_file).await.map(|_| ())
    }

    pub async fn upload_to(&self, in_schema: InFileSchema, user: &AuthUser) -> PentaractResult<()> {
        // 0. path validation
        if !Self::validate_filepath(&in_schema.path) {
            return Err(PentaractError::InvalidPath);
        }

        let in_file = InFile::new(in_schema.path, in_schema.size, in_schema.storage_id);

        // 1. saving file to db
        let file = self.repo.create_file(in_file).await?;

        self._upload(file, in_schema.file, user).await
    }

    pub async fn upload_anyway(
        &self,
        in_file: InFile,
        file_data: Bytes,
        user: &AuthUser,
    ) -> PentaractResult<()> {
        // 1. saving file in db
        let file = self.repo.create_file_anyway(in_file).await?;

        self._upload(file, file_data, user).await
    }

    async fn _upload(&self, file: File, file_data: Bytes, user: &AuthUser) -> PentaractResult<()> {
        // 2. sending file to storage manager
        let (resp_tx, resp_rx) = oneshot::channel();

        let message = {
            let upload_file_data = UploadFileData {
                file_id: file.id,
                user_id: user.id,
                file_data: file_data.as_ref().into(),
            };
            ClientMessage {
                data: ClientData::UploadFile(upload_file_data),
                tx: resp_tx,
            }
        };

        tracing::debug!("sending task to manager");
        let _ = self.tx.send(message).await;

        // 3. waiting for a storage manager result
        let message_back = match resp_rx.await.unwrap().data {
            StorageManagerData::UploadFile(r) => r,
            _ => unimplemented!(),
        };
        if let Err(e) = message_back.or({
            tracing::debug!("file loaded successfully");

            // 4. setting file as uploaded
            self.repo.set_as_uploaded(file.id).await
        }) {
            tracing::error!("{e}");

            // fallback logic: deleting file
            let _ = self.repo.delete_with_folders(file.id).await;

            return Err(e);
        };

        Ok(())
    }

    pub async fn download(
        &self,
        path: &str,
        storage_id: Uuid,
        user: &AuthUser,
    ) -> PentaractResult<Vec<u8>> {
        // 0. path validation
        if !Self::validate_path(path) {
            return Err(PentaractError::InvalidPath);
        }

        // 1. getting file by path
        let file = self.repo.get_file_by_path(path, storage_id).await?;

        // 2. sending task to storage manager
        let (resp_tx, resp_rx) = oneshot::channel();

        let message = {
            let download_file_data = DownloadFileData {
                file_id: file.id,
                storage_id,
                user_id: user.id,
            };
            ClientMessage {
                data: ClientData::DownloadFile(download_file_data),
                tx: resp_tx,
            }
        };

        tracing::debug!("sending task to manager");
        let _ = self.tx.send(message).await;

        // 3. waiting for a storage manager result
        match resp_rx.await.unwrap().data {
            StorageManagerData::DownloadFile(r) => r,
            _ => unimplemented!(),
        }
    }

    pub async fn list_dir(self, storage_id: Uuid, path: &str) -> PentaractResult<Vec<FSElement>> {
        self.repo.list_dir(storage_id, path).await
    }

    pub async fn delete(
        &self,
        path: &str,
        storage_id: Uuid,
        _user: &AuthUser,
    ) -> PentaractResult<()> {
        // 0. path validation
        if !Self::validate_path(path) {
            return Err(PentaractError::InvalidPath);
        }

        // 1. deleting file
        self.repo.delete(path, storage_id).await
    }

    /////////////////////////////////////////////////////////////////////
    ////    Helpers
    /////////////////////////////////////////////////////////////////////

    fn validate_filepath(path: &str) -> bool {
        Self::validate_path(path) && !path.ends_with(r"/")
    }

    fn validate_path(path: &str) -> bool {
        !path.starts_with(r"/") && !path.contains(r"//")
    }
}
