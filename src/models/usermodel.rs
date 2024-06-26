use alloy::primitives::Bytes;
use ethereum_types::H160;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PrimitiveData{
    pub address:H160,
    pub data:LogData
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LogData{
    pub topics: String,
    pub data: Bytes
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub inner:PrimitiveData,
    pub block_hash: String,
    pub block_number: String,
    pub block_timestamp: Option<String>,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub log_index: String,
    pub removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    pub address:String,
    pub event_name: String,
    pub block_number: u64,
}