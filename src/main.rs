#[macro_use] extern crate rocket;
use rocket_db_pools::Database;

pub mod db;
pub mod routes;



#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::MongoClient::init())
    .mount("/", routes![routes::index])
}