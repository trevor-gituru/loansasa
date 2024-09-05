use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser <'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginForm {
    pub identifier: String,
    pub password: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{\n\tid: {},\n\tname:\
        {},\n\temail: {},\n\tpassword: {},\n\tcreated_at: {}\n}}", self.id, self.name, self.email, 
        self.password, self.created_at)
    }
}