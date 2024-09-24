use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub initial_balance: String,
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub balance: Option<u128>,  // Assuming balance can be null
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct Wallet {
    pub id: i32,
    pub account_address: String,
    pub private_key: String,
    pub public_key: String,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::wallets)]
pub struct NewWallet <'a> {
    pub account_address: &'a str,
    pub public_key: &'a str,
    pub private_key: &'a str,
    pub user_id: i32
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wallet {{\n\tid: {},\n\taccount_address: \
        {},\n\tpublic_key: {},\n\tprivate_key: {},\n\tuser_id: {}\n}}", self.id, 
        self.account_address, self.public_key, 
        self.private_key, self.user_id.unwrap_or(0_i32))
    }
}