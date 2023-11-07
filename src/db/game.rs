use rocket_db_pools::{mongodb::Collection, Connection};

use crate::models::game::{Game, InsertGameResponse};

use super::{MongoClient, Result};

pub async fn insert_game(db: Connection<MongoClient>, game: Game) -> Result<InsertGameResponse> {
    let client = db.into_inner();
    let database = client.database("GameData");
    let collection: Collection<Game> = database.collection("Games");
    let insert_result = collection.insert_one(game, None).await?;
    Ok(InsertGameResponse {
        game_id: insert_result
            .inserted_id
            .as_object_id()
            .unwrap()
            .to_string(),
    })
}
