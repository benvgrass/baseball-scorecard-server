pub mod game;

use crate::db::{MongoClient, Result};
use bson::{doc, oid::ObjectId};
use rocket::futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::mongodb::{Client, Collection, Database};
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDocument {
    pub _id: ObjectId,
    pub name: String,
    pub short_name: String,
}

#[get("/")]
pub async fn index(db: Connection<MongoClient>) -> Result<Json<ResponseDocument>> {
    let client: Client = db.into_inner();
    let database: Database = client.database("GameData");
    let coll: Collection<ResponseDocument> = database.collection("Games");
    let result: Json<ResponseDocument> = coll
        .find_one(doc! {"short_name": "hello"}, None)
        .map_ok(|r| Json(r.unwrap()))
        .await?;
    Ok(result)
}
