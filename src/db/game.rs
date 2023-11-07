use rocket_db_pools::{mongodb::Collection, Connection};

use crate::models::game::{Game, InsertGameResponse};

use super::{MongoClient, Result};

/// Inserts a new game into MongoDB database.
///
/// # Arguments
///
/// * `db` - A `Connection` object representing the MongoDB client connection.
/// * `game` - A `Game` object representing the game to be inserted.
///
/// # Returns
///
/// Returns a `Result` containing an `InsertGameResponse` object with the ID of the inserted game.
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
