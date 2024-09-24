use crate::models::wallets::{NewWallet, Wallet, Account};
use diesel::prelude::*;
use diesel::{QueryResult, insert_into};
use reqwest::Client;


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

    wallets.filter(user_id.eq(users_id)).first(conn)
}

pub fn wallet_exists(
    wallet_address: &str, 
    conn: &mut PgConnection
) -> bool{
    use crate::schema::wallets::dsl::*;

    // Check if wallet exists
    
        let result: QueryResult<Wallet> = wallets
            .filter(account_address.eq(wallet_address))
            .first(conn);
            
        match result{
            Ok(_) => true,
            _ => false
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


pub async fn setup_account(conn: &mut PgConnection){

    // Create an HTTP client
    let client = Client::new();

   
    // Send the POST request
    let response = client
        .get("http://127.0.0.1:5050/predeployed_accounts") 
        .send()  // Send the request
        .await
        .unwrap(); // Await the response

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let response_json: Vec<Account> = response.json().await.unwrap();
        // Save account to dB
        for account in &response_json {
            if !(wallet_exists(&account.address, conn)){
                let new_wallet: NewWallet<'_> = NewWallet {
                    account_address: &account.address,
                    public_key: &account.public_key,
                    private_key: &account.private_key,
                    user_id: 1_i32
                };
                let wallet = create_wallet(&new_wallet, conn).unwrap();
                println!("Successfully added wallet:\n{wallet}");
            }
        }

    } else {
        eprintln!("Failed to fetch predeployed accounts from starknet: {:?}", &response.status());

    }

}
