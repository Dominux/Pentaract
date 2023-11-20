use askama::Template;

use crate::models::storages::Storage;

#[derive(Template)]
#[template(path = "storages/index.jinja")]
pub struct StoragesIndexTemplate {
    storages: Vec<Storage>,
}

impl StoragesIndexTemplate {
    pub fn new(storages: Vec<Storage>) -> Self {
        Self { storages }
    }
}

#[derive(Template)]
#[template(path = "storages/list.jinja")]
pub struct StoragesListTemplate {
    storages: Vec<Storage>,
}

impl StoragesListTemplate {
    pub fn new(storages: Vec<Storage>) -> Self {
        Self { storages }
    }
}
