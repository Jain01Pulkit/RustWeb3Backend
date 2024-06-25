// use std::env;
// extern crate dotenv;
// use alloy::{rpc::types::Log};
// use dotenv::dotenv;

// use crate::models::eventsModel::Events;
// use mongodb::{
//     bson::{doc, extjson::de::Error, oid::ObjectId},
//     results::{DeleteResult, InsertOneResult,InsertManyResult, UpdateResult},
//     sync::{Client, Collection},
// };

// pub struct MongoRepo {
//     col: Collection<alloy::rpc::types::Log>,
// }

// impl MongoRepo {
//     pub fn init() -> Self {
//         dotenv().ok();
//         let uri = match env::var("MONGOURI") {
//             Ok(v) => v.to_string(),
//             Err(_) => format!("Error loading env variable"),
//         };
//         let client = Client::with_uri_str(uri).unwrap();
//         let db = client.database("rustDB");
//         let col: Collection<alloy::rpc::types::Log> = db.collection("User");
//         MongoRepo { col }
//     }
   
//     pub fn create_events(&self, new_user: Vec<alloy::rpc::types::Log>) -> Result<InsertManyResult, Error> {
        
//         let user = self
//             .col
//             .insert_many(&new_user, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)
//     }
// }
