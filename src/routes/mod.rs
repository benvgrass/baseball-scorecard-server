use crate::db::MongoClient;
use rocket::futures::TryFutureExt;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::{Client, Collection, Database};
use rocket_db_pools::Connection;
use rocket_db_pools::{mongodb, mongodb::bson::doc};
use serde::{Deserialize, Serialize};
// use rocket_db_pools::mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    _id: Option<i64>,
    name: String,
    short_name: String,
}

type Result<T, E = Debug<mongodb::error::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index(mut db: Connection<MongoClient>) -> Result<Json<Response>> {
    let client: Client = db.into_inner();
    let database: Database = client.database("GameData");
    let coll: Collection<Response> = database.collection("Games");
    let result = coll
        .find_one(doc! {"short_name": "hello"}, None)
        .map_ok(|r| Json(r.unwrap()))
        .await?;

    println!("{:?}", result);
    Ok(result)
}
