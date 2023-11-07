use rocket::response::Debug;
use rocket_db_pools::{
    mongodb::{error::Error, Client},
    Database,
};

pub mod game;

/// A type alias for `std::result::Result` with a default error type of `Debug<mongodb::error::Error>`.
pub type Result<T, E = Debug<Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("mongodb")]
pub struct MongoClient(Client);
