use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use reqwest::Client;
// use primitive_types::{H160, U256};

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Params,
    id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Params {
    None(EmptyParams),
    Filter(Filter), // When filter is applied
}
// Define an empty struct for the empty variant
#[derive(Serialize, Deserialize, Debug)]
struct EmptyParams;

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    jsonrpc: String,
    id: u32,
    result: Option<ResultType>, // Result is optional to handle the case when error might occur
    // error: Option<RpcError>,    // Error in case of failure
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultType {
    Int(u32),
    Events(EventData)
}
#[derive(Serialize, Deserialize, Debug)]
struct EventData {
    events: Vec<Event>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Event {
    transaction_hash: String,
    block_hash: String,
    block_number: u32,
    from_address: String,
    keys: Vec<String>,
    data: Vec<String>,
}
// #[derive(Serialize, Deserialize, Debug)]
// struct ErrorResponse {
//     jsonrpc: String,
//     id: u32,
//     error: ErrorDetails,
// }

pub async fn starknet_block() -> HttpResponse {

    // Create an HTTP client
    let client = Client::new();

    // Define the JSON body for the POST request
    let request_body = RpcRequest{
        jsonrpc: String::from("2.0"),
        method: String::from("starknet_blockNumber"),
        params: Params::None(EmptyParams),
        id: 1
      };

    // Send the POST request
    let response = client
        .post("http://127.0.0.1:5050") // Replace with your API URL
        .json(&request_body)  // Attach the JSON body
        .send()  // Send the request
        .await
        .unwrap(); // Await the response

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let response_json: RpcResponse = response.json().await.unwrap();
        println!("Response: {:?}", &response_json);
        HttpResponse::Ok().body(format!("Latest RpcResponse is: {:?}", &response_json))

    } else {
        eprintln!("Failed to send POST request: {:?}", &response.status());
        HttpResponse::Ok().body(format!("Failed to send POST request: {:?}", &response.status()))

    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    initial_balance: String,
    address: String,
    public_key: String,
    private_key: String,
    balance: Option<u128>,  // Assuming balance can be null
}
pub async fn starknet_account() -> HttpResponse {

    // Create an HTTP client
    let client = Client::new();

   
    // Send the POST request
    let response = client
        .get("http://127.0.0.1:5050/predeployed_accounts") // Replace with your API URL
        .send()  // Send the request
        .await
        .unwrap(); // Await the response

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let response_json: Vec<Account> = response.json().await.unwrap();
        // Print the accounts
        for account in &response_json {
            println!("Account: {:?}", account.address);
        }
        HttpResponse::Ok().body(format!("Response: {:?}", &response_json))

    } else {
        eprintln!("Failed to send POST request: {:?}", &response.status());
        HttpResponse::Ok().body(format!("Failed to send POST request: {:?}", &response.status()))

    }

}

// Define the Filter struct
#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    filter: FilterParams, // Wrapping in a `filter` field
}
#[derive(Serialize, Deserialize, Debug)]
struct FilterParams {
    from_block: Block,
    to_block: ParamBlock,
    address: String,
    keys: Vec<Vec<String>>,
    chunk_size: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ParamBlock {
    Str(String),
    Struct(Block), // When filter is applied
}
#[derive(Serialize, Deserialize, Debug)]
struct Block {
    block_number: u32,
}

pub async fn starknet_events() -> HttpResponse {

    // Create an HTTP client
    let client = Client::new();
    let filter_params = RpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "starknet_getEvents".to_string(),
        params: Params::Filter(Filter {
            filter: FilterParams {
                from_block: Block { block_number: 0 },
                to_block: ParamBlock::Str("latest".to_string()),
                address: "0x28934222463ba74aa2fbcd744e9b1c3445bf759913fffe4a996e90aa3079aec".to_string(),
                keys: vec![vec!["0x99cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9".to_string()], vec![], 
                vec!["0x302a349b229b085fe5fccaa2c54548458f87ddf66e2f0a3e007a8466eeed63a".to_string()]],
                chunk_size: 100,
            },
        }),
        id: 1,
    };
    // let filter = Filter {
    //     from_block: Block { block_number: 0 },
    //     to_block: Block { block_number: 13 },
    //     address: "0x71207f391e2f3796492d94539519df1a2f7b68cb1245f9050ba6cf860ad2ee8".to_string(),
    //     keys: vec![],
    //     chunk_size: 100,
    // };
    // // Define the JSON body for the POST request
    // let request_body = RpcRequest{
    //     jsonrpc: String::from("2.0"),
    //     method: String::from("starknet_blockNumber"),
    //     params: Params::Filter(filter),
    //     id: 1
    //   };
    // let request_body = json!({
    //     "jsonrpc": "2.0",
    //     "method": "starknet_getEvents",
    //     "params": {
    //         "filter": {
    //             "from_block": { "block_number": 0 },
    //             "to_block": { "block_number": 13 },
    //             "address": "0x71207f391e2f3796492d94539519df1a2f7b68cb1245f9050ba6cf860ad2ee",
    //             "keys": [],
    //             "chunk_size": 100
    //         }
    //     },
    //     "id": 1
    // });
    // Send the POST request
    let response = client
        .post("http://127.0.0.1:5050") // Replace with your API URL
        .json(&filter_params)  // Attach the JSON body
        .send()  // Send the request
        .await
        .unwrap(); // Await the response

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let response_json:  RpcResponse = response.json().await.unwrap();
        println!("Response: {:?}", &response_json);
        if let ResultType::Events(data) = &response_json.result.as_ref().unwrap() {
            return HttpResponse::Ok().json(&data.events);

        }
        HttpResponse::Ok().body(format!("Latest RpcResponse is: {:?}", &response_json.result.unwrap()))

    } else {
        eprintln!("Failed to send POST request: {:?}", &response.status());
        HttpResponse::Ok().body(format!("Failed to send POST request: {:?}", &response.status()))

    }
}