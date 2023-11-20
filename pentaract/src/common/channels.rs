use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::errors::PentaractResult;

//////////////////////////////////////
///     Client schemas
//////////////////////////////////////

pub struct ClientMessage {
    pub tx: StorageManagerSender,
    pub data: ClientData,
}

pub enum ClientData {
    UploadFile(UploadFileData),
    DownloadFile(DownloadFileData),
}

pub struct UploadFileData {
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub file_data: Box<[u8]>,
}

pub struct DownloadFileData {
    pub file_id: Uuid,
    pub storage_id: Uuid,
    pub user_id: Uuid,
}
//////////////////////////////////////
///     Storage manager schemas
//////////////////////////////////////

pub struct StorageManagerMessage {
    pub data: StorageManagerData,
}

impl StorageManagerMessage {
    pub fn new(data: StorageManagerData) -> Self {
        Self { data }
    }
}

pub enum StorageManagerData {
    UploadFile(PentaractResult<()>),
    DownloadFile(PentaractResult<Vec<u8>>),
}

//////////////////////////////////////
///     Channels
//////////////////////////////////////

// pub type ClientListener = oneshot::Receiver<StorageManagerMessage>;
pub type StorageManagerSender = oneshot::Sender<StorageManagerMessage>;
pub type ClientSender = mpsc::Sender<ClientMessage>;
pub type StorageManagerListener = mpsc::Receiver<ClientMessage>;
