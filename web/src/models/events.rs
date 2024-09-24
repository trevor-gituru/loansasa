use serde::{Deserialize, Serialize};

// Requests
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: ParamsRequest,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParamsRequest {
    None(EmptyParams),
    Filter(Filter), // When filter is applied
}
// Define an empty struct for the empty variant
#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyParams;

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub filter: FilterParams, // Wrapping in a `filter` field
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FilterParams {
    pub from_block: Block,
    pub to_block: ParamBlock,
    pub address: String,
    pub keys: Vec<Vec<String>>,
    pub chunk_size: u32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParamBlock {
    Str(String),
    Struct(Block), // When filter is applied
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_number: u32,
}

// Response section
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: Option< RpcResultType>, // Result is optional to handle the case when error might occur
    pub error: Option<RpcError>,    // Error in case of failure
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum  RpcResultType {
    Events(EventData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventData {
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub transaction_hash: String,
    pub block_hash: String,
    pub block_number: u32,
    pub from_address: String,
    pub keys: Vec<String>,
    pub data: Vec<String>,
}