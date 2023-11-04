use crate::db::MongoClient;
use rocket::futures::TryFutureExt;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use bson::{doc, oid::ObjectId};
use rocket_db_pools::mongodb;
use rocket_db_pools::mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDocument {
    pub _id: ObjectId,
    pub name: String,
    pub short_name: String,
}

type Result<T, E = Debug<mongodb::error::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index(db: Connection<MongoClient>) -> Result<Json<ResponseDocument>> {
    let client: Client = db.into_inner();
    let database: Database = client.database("GameData");
    let coll: Collection<ResponseDocument> = database.collection("Games");
    let result: Json<ResponseDocument> = coll.find_one(doc! {"short_name": "hello"}, None)
        .map_ok(|r| {
            Json(r.unwrap())
        }).await?;
    Ok(result)
}
