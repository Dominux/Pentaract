use askama::Template;

#[derive(Template)]
#[template(path = "login.jinja")]
pub struct LoginTemplate<'a> {
    error: Option<&'a str>,
}

impl<'a> LoginTemplate<'a> {
    pub fn new(error: Option<&'a str>) -> Self {
        Self { error }
    }
}

impl<'a> Default for LoginTemplate<'a> {
    fn default() -> Self {
        Self { error: None }
    }
}
