use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::errors::PentaractResult;

pub enum Method {
    UploadFile(UploadFileData),
}

pub struct UploadFileData {
    pub file_id: Uuid,
    pub user_id: Uuid,
}

pub struct ClientMessage {
    pub tx: StorageManagerSender,
    pub method: Method,
}

pub type ClientListener = oneshot::Receiver<PentaractResult<()>>;
pub type StorageManagerSender = oneshot::Sender<PentaractResult<()>>;
pub type ClientSender = mpsc::Sender<ClientMessage>;
pub type StorageManagerListener = mpsc::Receiver<ClientMessage>;
