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
    block_number: u32,
}
