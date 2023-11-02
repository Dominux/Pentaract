use askama::Template;

#[derive(Template)]
#[template(path = "storages/id.html")]
pub struct StorageTemplate<'a> {
    id: uuid::Uuid,
    name: &'a str,
}

impl<'a> StorageTemplate<'a> {
    pub fn new(id: uuid::Uuid, name: &'a str) -> Self {
        Self { id, name }
    }
}
