use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct Users {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser <'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
}