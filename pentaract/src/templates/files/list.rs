use askama::Template;

use crate::models::files::FSElement;

#[derive(Template)]
#[template(path = "files/list.jinja")]
pub struct FilesListTemplate<'a> {
    id: uuid::Uuid,
    path: &'a str,
    fs_layer: Vec<FSElement>,
}

impl<'a> FilesListTemplate<'a> {
    pub fn new(id: uuid::Uuid, path: &'a str, fs_layer: Vec<FSElement>) -> Self {
        Self { id, path, fs_layer }
    }
}
