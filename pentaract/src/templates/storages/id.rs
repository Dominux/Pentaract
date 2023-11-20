use askama::Template;

use crate::models::files::FSElement;

#[derive(Template)]
#[template(path = "storages/id.jinja")]
pub struct StorageTemplate<'a> {
    id: uuid::Uuid,
    name: &'a str,
    fs_layer: Vec<FSElement>,
}

impl<'a> StorageTemplate<'a> {
    pub fn new(id: uuid::Uuid, name: &'a str, fs_layer: Vec<FSElement>) -> Self {
        Self { id, name, fs_layer }
    }
}
