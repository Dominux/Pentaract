use tokio::sync::{mpsc, oneshot};

pub type ChannelMessage = String;
pub type ClientListener = oneshot::Receiver<ChannelMessage>;
pub type StorageManagerSender = oneshot::Sender<ChannelMessage>;
pub type ClientSender = mpsc::Sender<StorageManagerSender>;
pub type StorageManagerListener = mpsc::Receiver<StorageManagerSender>;
