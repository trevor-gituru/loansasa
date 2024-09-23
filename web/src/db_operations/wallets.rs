use crate::models::wallets::{NewWallet, Wallet};
use diesel::prelude::*;
use diesel::{QueryResult, insert_into};

pub fn create_wallet(
    new_wallet: &NewWallet, 
    conn: &mut PgConnection
) -> QueryResult<Wallet>{
    use crate::schema::wallets::dsl::*;

     // normal diesel operations
     insert_into(wallets)
     .values(new_wallet)
     .get_result::<Wallet>(conn)

}

pub fn find_wallet(
    users_id: i32, 
    conn: &mut PgConnection
) -> QueryResult<Wallet>{
    use crate::schema::wallets::dsl::*;

    // Check if user id is above 0
    if users_id != 0_i32 {
        return wallets.filter(user_id.eq(users_id)).first(conn);
    }else{
        return wallets.filter(user_id.is_null()).first(conn);
    }
}

pub fn assign_wallet(
    wallet_id: i32,
    users_id: i32,
    conn: &mut PgConnection
) -> QueryResult<Wallet>{
    use crate::schema::wallets::dsl::*;

    diesel::update(wallets.filter(id.eq(wallet_id)))
    .set((
        user_id.eq(users_id),
    ))
    .get_result::<Wallet>(conn)

}