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

#[cfg(test)]
mod tests {
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use super::rocket;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);    }
}
