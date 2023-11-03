use rocket_db_pools::{mongodb, Database};

#[derive(Database)]
#[database("mongodb")]
pub struct MongoClient(mongodb::Client);