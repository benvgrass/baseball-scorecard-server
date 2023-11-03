use rocket_db_pools::{mongodb, Database};

pub mod game;

#[derive(Database)]
#[database("mongodb")]
pub struct MongoClient(mongodb::Client);
