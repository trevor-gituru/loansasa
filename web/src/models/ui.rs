use askama::Template;

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate<'a>{
    pub name: &'a str,
    pub email: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate<'a>{
    pub identifier: &'a str,
    pub error: Option<&'a str>,
}
#[derive(Template)]
#[template(path = "dashboard/home.html")]
pub struct HomeTemplate{}

#[derive(Template)]
#[template(path = "dashboard/profile.html")]
pub struct ProfileTemplate<'a>{
    pub account_address: &'a str,
    pub private_key: &'a str,
    pub error: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "dashboard/loans.html")]
pub struct LoansTemplate{}

#[derive(Template)]
#[template(path = "dashboard/lenders.html")]
pub struct LendersTemplate{}