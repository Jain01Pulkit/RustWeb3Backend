use alloy::primitives::Bytes;
use ethereum_types::H160;
use events::{consts::CRON_EXPRESSION_15_SEC, cron_util};
use serde::{Deserialize, Serialize};
use service::userservice::MongoRepo;
use tokio::runtime::Runtime;
mod models;
mod service;
#[derive(Serialize, Deserialize, Debug)]
struct LogData {
    pub topics: String,
    pub data: Bytes,
}
#[derive(Serialize, Deserialize, Debug)]

struct PrimitiveData {
    pub address: H160,
    pub data: LogData,
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
    cron_util::create_cronjob_with_schedule(CRON_EXPRESSION_15_SEC, produce_joke);
}
fn produce_joke() {
    let rt = Runtime::new().unwrap();
    let db = MongoRepo::init();
    rt.block_on(async {
        match MongoRepo::get_events(
            &db,
            "Transfer",
            38159350,
            "0xfC0b3e6D09566bA2Bb5F069Da59390EA001904Fb",
        )
        .await
        {
            Ok((events, end_block)) => {
                match MongoRepo::insert_events(&db, events, end_block as i64) {
                    Ok(_) => println!("Events inserted successfully"),
                    Err(e) => println!("Error inserting events: {}", e),
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    });
}
