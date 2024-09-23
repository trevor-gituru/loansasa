use diesel::prelude::*;
use std::fmt;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct Wallet {
    pub id: i32,
    pub starknet_address: String,
    pub private_key: String,
    pub public_key: String,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::wallets)]
pub struct NewWallet <'a> {
    pub starknet_address: &'a str,
    pub public_key: &'a str,
    pub private_key: &'a str,
    pub user_id: i32
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wallet {{\n\tid: {},\n\tstarknet_address:\
        {},\n\tpublic_key: {},\n\tprivate_key: {},\n\tuser_id: {}\n}}", self.id, 
        self.starknet_address, self.public_key, 
        self.private_key, self.user_id.unwrap_or(0_i32))
    }
}