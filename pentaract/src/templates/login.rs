use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

impl LoginTemplate {
    pub fn new() -> Self {
        Self
    }
}
