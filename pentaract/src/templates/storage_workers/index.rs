use askama::Template;

use crate::models::storage_workers::StorageWorker;

#[derive(Template)]
#[template(path = "storage_workers/index.html")]
pub struct StorageWorkersIndexTemplate {
    storage_workers: Vec<StorageWorker>,
}

impl StorageWorkersIndexTemplate {
    pub fn new(storage_workers: Vec<StorageWorker>) -> Self {
        Self { storage_workers }
    }
}

#[derive(Template)]
#[template(path = "storage_workers/list.html")]
pub struct StorageWorkersListTemplate {
    storage_workers: Vec<StorageWorker>,
}

impl StorageWorkersListTemplate {
    pub fn new(storage_workers: Vec<StorageWorker>) -> Self {
        Self { storage_workers }
    }
}