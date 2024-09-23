
use actix_web::{HttpResponse, Responder, HttpRequest, web};
use crate::db_operations::session::generate_session_id;
use crate::models::app_state::AppState;
use crate::models::session::Session;
use crate::utils::client_info::{get_browser, get_ip};
use deadpool_redis::redis::AsyncCommands;
use serde_json::{to_string, from_str};

use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Felt, FunctionCall},
        utils::get_selector_from_name,
    },
    macros::{felt, selector},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
    signers::{LocalWallet, SigningKey},

};
use starknet::accounts::Call;
fn hex_to_utf8(hex_str: &str) -> Result<String, std::string::FromUtf8Error> {
    // Remove '0x' prefix if present
    let hex_str = hex_str.trim_start_matches("0x");

    // Convert the hex string into a vector of bytes
    let bytes = (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i+2], 16).unwrap())
        .collect::<Vec<u8>>();

    // Convert bytes to UTF-8 string
    String::from_utf8(bytes)
}
pub async fn starknet_mint() -> HttpResponse {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("http://127.0.0.1:5050").unwrap(),
    ));

    let tst_token_address =
        felt!("0x3195eba89affafa461168eac0d402d5c553d9aebe3a58811523c1552d1ca131");

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("0x00000000000000000000000000000000c658823cde5cfc45d55073a49abe388e").unwrap(),
    ));
    let address = Felt::from_hex("0x2624b07badd840b4ed1219fc3b8de3fd39c8d4b833d3518a83cb8d825f6d5ca").unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute_v1(vec![Call {
            to: tst_token_address,
            selector: get_selector_from_name("add_user").unwrap(),
            calldata: vec![
                Felt::from_hex("0x4162656C").unwrap(),
            ],
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", &result.transaction_hash);
    HttpResponse::Ok().body(format!("Transaction hash: {:#064x}", &result.transaction_hash))
}
pub async fn starknet_get() -> HttpResponse{
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("http://127.0.0.1:5050").unwrap(),
    ));

    let tst_token_address =
        felt!("0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7");

    let call_result =
        provider
            .call(
                FunctionCall {
                    contract_address: tst_token_address,
                    entry_point_selector: selector!("get_user"),
                    calldata: vec![
                        Felt::from_hex("0x2624b07badd840b4ed1219fc3b8de3fd39c8d4b833d3518a83cb8d825f6d5ca").unwrap()
                        ],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .expect("failed to call contract");
    
    let hex_str = call_result.get(0).unwrap().to_hex_string();
    // Convert bytes to UTF-8 string
    let call_result = hex_to_utf8(&hex_str).unwrap();

    dbg!(&call_result);
    HttpResponse::Ok().body(format!("User: {:?}", &call_result))
}

pub async fn client(req: HttpRequest) -> impl Responder {
    let client_browser = get_browser(&req).unwrap_or_else(|| "unknown browser".to_string());
    let client_ip = get_ip(&req).unwrap_or_else(|| "unknown".to_string());
    HttpResponse::Ok().body(format!("Client IP: {}\nClient Browser: {}", client_ip, client_browser))
}

pub async fn test_redis(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let client_browser = get_browser(&req);
    let client_ip = get_ip(&req);
    // Get a connection from the pool
    let mut conn = app_state.redis_pool.get().await.expect("Failed to get redis connection");
    let session_id = generate_session_id(&mut conn);
    let client_session = Session::new(session_id.await, 5,client_ip, client_browser);

    // Use the connection to interact with Redis
    let _: () = conn.set_ex(
        format!("session: {}", client_session.session_id), 
        to_string(&client_session).unwrap(), 
        5)
        .await.
        unwrap();
    let result: String = conn.get(
        format!("session: {}", client_session.session_id)
    )
        .await
        .unwrap();
    let sess: Session = from_str(&result).unwrap();
    let exis: bool = conn.exists(format!("session: {}", client_session.session_id)).await.expect("Error check existance");

    HttpResponse::Ok().body(format!("Got value from Redis: {}\n\n{}", sess.session_id, exis))

        
}
