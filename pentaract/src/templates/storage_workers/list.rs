use askama::Template;

#[derive(Template)]
#[template(path = "storage_workers/list.html")]
pub struct StorageWorkersListTemplate<'a> {
    name: Option<&'a str>,
}

impl<'a> StorageWorkersListTemplate<'a> {
    pub fn new(name: Option<&'a str>) -> Self {
        Self { name }
    }
}
