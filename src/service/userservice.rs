use std::{env, error::Error, str::FromStr};
extern crate dotenv;
use crate::models::usermodel::BlockInfo;
use crate::models::usermodel::{Events, LogData, PrimitiveData};
use alloy::providers::Provider;
use alloy::{primitives::address, providers::ProviderBuilder, rpc::types::Filter, sol};
use dotenv::dotenv;
use ethereum_types::H160;
use mongodb::{
    bson::doc,
    results::InsertManyResult,
    sync::{Client, Collection},
};
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TOKEN,
    "src/token.json"
);

pub struct MongoRepo {
    col: Collection<Events>,
    col_block: Collection<BlockInfo>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Events> = db.collection("User");
        let col_block: Collection<BlockInfo> = db.collection("Block");

        MongoRepo { col, col_block }
    }

    pub fn insert_events(
        &self,
        events: Vec<Events>,
        end_block: i64,
    ) -> Result<Option<InsertManyResult>, Box<dyn Error>> {
        print!("{:?}kkkkkkkkkkkkkkkkkkkk", events);
        if !events.is_empty() {
            let user = self
                .col
                .insert_many(&events, None)
                .ok()
                .expect("Error creating user");
            Ok(Some(user))
        } else {
            let filter = doc! { "event_name": "Transfer" };
            let update = doc! { "$set": { "block_number": end_block as i64} };
            self.col_block.update_one(filter, update, None)?;
            Ok(None)
        }
    }

    pub async fn get_events(
        &self,
        event_name: &str,
        contract_start_block: u64,
        contract_address: &str,
    ) -> Result<(Vec<Events>, u64), Box<dyn Error>> {
        // let collection = db.collection::<BlockInfo>("blockInfo");

        let filter = doc! { "event_name": "Transfer" };
        let block_info = self.col_block.find_one(filter, None)?;

        let start_block = if let Some(block_info) = block_info {
            block_info.block_number + 1
        } else {
            self.col_block.insert_one(
                BlockInfo {
                    address: contract_address.to_string(),
                    event_name: event_name.to_string(),
                    block_number: contract_start_block,
                },
                None,
            )?;
            contract_start_block
        };
        let rpc_url = "https://bsc-testnet.blockpi.network/v1/rpc/public".parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        // Create a filter to get all logs from the latest block.
        let current_block = provider.get_block_number().await?;
        let mut transformed_logs = Vec::new();
        let end_block = 12;
        if current_block >= start_block {
            let end_block = if start_block + 1000 > current_block {
                current_block
            } else {
                start_block + 1000
            };
            print!("{},{},{}", current_block, end_block, start_block);
            let filter = Filter::new()
                .address(address!("fC0b3e6D09566bA2Bb5F069Da59390EA001904Fb"))
                .event("Transfer(address,address,uint256)")
                .from_block(start_block)
                .to_block(end_block);
            let logs = provider.get_logs(&filter).await?;
            for log in logs {
                let log_wrapper = Events {
                    inner: PrimitiveData {
                        address: H160::from_str(&log.inner.address.to_string())?,
                        data: LogData {
                            topics: "jjjj".to_string(),
                            data: log.inner.data.data, // Convert Bytes to hex string
                        },
                    },
                    block_hash: log.block_hash.unwrap_or_default().to_string(),
                    block_number: log.block_number.unwrap_or_default().to_string(),
                    block_timestamp: log.block_timestamp.map(|ts| ts.to_string()),
                    transaction_hash: log.transaction_hash.unwrap_or_default().to_string(),
                    transaction_index: log.transaction_index.unwrap_or_default().to_string(),
                    log_index: log.log_index.unwrap_or_default().to_string(),
                    removed: log.removed,
                };
                transformed_logs.push(log_wrapper);
            }
            print!("{},{}", event_name, end_block);
            // Ok(transformed_logs)
            // let filter = doc! { "event_name": event_name };
            // let update = doc! { "$set": { "block_number": end_block as i64} };
            // self.col_block.update_one(filter, update, None)?;
            Ok((transformed_logs, end_block))
        } else {
            println!("Waiting for blocks to update");
            Ok((transformed_logs, end_block))
        }
    }
}
