use rocket_db_pools::{mongodb::{Client, error::Error}, Database};
use rocket::response::Debug;

pub mod game;

pub type Result<T, E = Debug<Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("mongodb")]
pub struct MongoClient(Client);