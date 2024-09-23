use diesel::prelude::*;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_details)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct UserDetails {
    pub id: i32,
    pub user_id: i32,
    pub account_address: String,
    pub private_key: String
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::user_details)]
pub struct NewUserDetails <'a> {
    pub user_id: i32,
    pub account_address: &'a str,
    pub private_key: &'a str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileForm {
    pub account_address: String,
    pub private_key: String,
}

impl fmt::Display for UserDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserDetails {{\n\tid: {},\n\tuser_id:\
        {},\n\taccount_address: {},\n\tprivate_key: {}\n}}", self.id, self.user_id, self.account_address, 
        self.private_key)
    }
}