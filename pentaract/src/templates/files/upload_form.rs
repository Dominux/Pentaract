use askama::Template;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "files/upload_form.html")]
pub struct UploadFormTemplate<'a> {
    pub storage_id: Uuid,
    pub path_err: Option<&'a str>,
    pub file_err: Option<&'a str>,
}

impl<'a> UploadFormTemplate<'a> {
    pub fn new(storage_id: Uuid, path_err: Option<&'a str>, file_err: Option<&'a str>) -> Self {
        Self {
            file_err,
            path_err,
            storage_id,
        }
    }
}