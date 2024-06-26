use alloy::{primitives::{Address, Bytes, FixedBytes}, signers::local::yubihsm::device::SerialNumber};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct PrimitiveData{
    address:Address,
    data:LogData
}
#[derive(Debug, Serialize, Deserialize)]
struct LogData{
    topics: Vec<FixedBytes<32>>,
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