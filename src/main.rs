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
    use crate::routes::ResponseDocument;

    use super::rocket;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);  
        let response_body: ResponseDocument = response.into_json().unwrap(); 
        assert_eq!(response_body._id.to_string(), "65448dd8383e995cfb9bc5cb".to_string());
        assert_eq!(response_body.name, "the quick brown fox".to_string());
        assert_eq!(response_body.short_name, "hello".to_string());
        
    }
}
