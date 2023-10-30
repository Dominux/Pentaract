use askama::Template;

#[derive(Template)]
#[template(path = "storages/create_form.html")]
pub struct StoragesCreateFormTemplate<'a> {
    name_err: Option<&'a str>,
}

impl<'a> StoragesCreateFormTemplate<'a> {
    pub fn new(name_err: Option<&'a str>) -> Self {
        Self { name_err }
    }
}

impl<'a> Default for StoragesCreateFormTemplate<'a> {
    fn default() -> Self {
        Self { name_err: None }
    }
}
