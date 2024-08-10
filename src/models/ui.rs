use askama::Template;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;