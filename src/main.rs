#[macro_use]
extern crate rocket;
use rocket_db_pools::Database;

pub mod db;
pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::MongoClient::init())
        .mount("/", routes![routes::index])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::Json;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
