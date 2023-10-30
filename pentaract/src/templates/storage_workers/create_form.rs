use askama::Template;

use crate::models::storages::Storage;

#[derive(Template)]
#[template(path = "storage_workers/create_form.html")]
pub struct StorageWorkersCreateFormTemplate<'a> {
    name_err: Option<&'a str>,
    token_err: Option<&'a str>,
    storages: Vec<Storage>,
}

impl<'a> StorageWorkersCreateFormTemplate<'a> {
    pub fn new(
        name_err: Option<&'a str>,
        token_err: Option<&'a str>,
        storages: Vec<Storage>,
    ) -> Self {
        Self {
            name_err,
            token_err,
            storages,
        }
    }
}
