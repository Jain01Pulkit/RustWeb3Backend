use std::{error::Error, str::FromStr};
use alloy::{
    primitives::{address, Bytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
    sol,
};
use ethereum_types::H160;
use events::{
    consts::CRON_EXPRESSION_5_SEC,
    cron_util,
};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
// mod service;
// mod models;
#[derive(Serialize, Deserialize, Debug)]
struct LogData {
    topics: String,
    data: Bytes,
}
#[derive(Serialize, Deserialize, Debug)]

struct PrimitiveData {
    address: H160,
    data: LogData,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogWrapper {
    inner: PrimitiveData,
    block_hash: String,
    block_number: String,
    block_timestamp: Option<String>,
    transaction_hash: String,
    transaction_index: String,
    log_index: String,
    removed: bool,
}

fn main() {
    // let db = MongoRepo::init();
    cron_util::create_cronjob_with_schedule(CRON_EXPRESSION_5_SEC, produce_joke);
}
fn produce_joke() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match fetchevents().await {
            Ok(joke) => println!("{:?}", joke),
            Err(e) => println!("Error: {}", e),
        }
    });
}
// #[tokio::main]
async fn fetchevents() -> Result<Vec<LogWrapper>, Box<dyn Error>> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    // Create a filter to get all logs from the latest block.
    let latest_block = provider.get_block_number().await?;
    // let filter = Filter::new()
    //     .from_block(latest_block - 1)
    //     .to_block(latest_block);

    // Get all logs from the latest block that match the filter.
    // Create a contract instance.
    println!("{}", latest_block);
    // let contract = TOKEN::new(
    //     "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?,
    //     &provider,
    // );
    // let logs = provider.get_logs(&filter).await?;
    let uniswap_token_address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
    // let filter = Filter::new().address(uniswap_token_address).from_block(latest_block);
    let filter = Filter::new()
        .address(uniswap_token_address)
        .event("Transfer(address,address,uint256)")
        .from_block(20167968);
    let logs = provider.get_logs(&filter).await?;
    let mut transformed_logs = Vec::new();

    for log in logs {
        let log_wrapper = LogWrapper {
            inner: PrimitiveData {
                address: H160::from_str(&log.inner.address.to_string())?,
                data: LogData {
                    topics: "jjjj".to_string(),
                    data: log.inner.data.data, // Convert Bytes to hex string
                },
            },
            block_hash: log.block_hash.unwrap_or_default().to_string(),
            block_number: log.block_number.unwrap_or_default().to_string(),
            block_timestamp: log.block_timestamp.map(|ts| ts.to_string()), // Convert Option<u64> to Option<String>
            transaction_hash: log.transaction_hash.unwrap_or_default().to_string(),
            transaction_index: log.transaction_index.unwrap_or_default().to_string(),
            log_index: log.log_index.unwrap_or_default().to_string(),
            removed: log.removed,
        };
        transformed_logs.push(log_wrapper);
    }
    print!("{:?}", transformed_logs);
    Ok(transformed_logs)
}
