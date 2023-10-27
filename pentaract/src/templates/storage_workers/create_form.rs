use askama::Template;

#[derive(Template)]
#[template(path = "storage_workers/create_form.html")]
pub struct StorageWorkersCreateFormTemplate<'a> {
    name_err: Option<&'a str>,
    token_err: Option<&'a str>,
}

impl<'a> StorageWorkersCreateFormTemplate<'a> {
    pub fn new(name_err: Option<&'a str>, token_err: Option<&'a str>) -> Self {
        Self {
            name_err,
            token_err,
        }
    }
}

impl<'a> Default for StorageWorkersCreateFormTemplate<'a> {
    fn default() -> Self {
        Self {
            name_err: None,
            token_err: None,
        }
    }
}
