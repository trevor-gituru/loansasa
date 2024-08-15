use askama::Template;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate<'a>{
    pub name: &'a str,
    pub email: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a>{
    pub identifier: &'a str,
    pub error: Option<&'a str>,
}