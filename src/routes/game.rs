use crate::{
    db::{game, MongoClient},
    models::game::{Game, InsertGameResponse},
};
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;

/// Inserts a new game into the database.
///
/// # Arguments
///
/// * `db` - A MongoDB connection.
/// * `game` - A JSON object representing the game to be inserted.
///
/// # Returns
///
/// A tuple containing a `Status` code and an optional JSON object representing the response.
#[post("/game", data = "<game>")]
pub async fn insert_game(
    db: Connection<MongoClient>,
    game: Json<Game>,
) -> (Status, Option<Json<InsertGameResponse>>) {
    if let Ok(game_response) = game::insert_game(db, game.into_inner()).await {
        (Status::Created, Some(Json(game_response)))
    } else {
        (Status::ServiceUnavailable, None)
    }
}

/// Retrieves a game from the database by its ID and returns it as JSON.
/// 
/// # Arguments
/// 
/// * `db` - A MongoDB connection.
/// * `game_id` - The ID of the game to retrieve.
/// 
/// # Returns
/// 
/// A tuple containing a `Status` code and an optional JSON object representing the game data.
/// If the game is found, returns a 200 OK status code with the game data.
/// If the game is not found, returns a 404 Not Found status code.
#[get("/game/<game_id>")]
pub async fn get_game(db: Connection<MongoClient>, game_id: String) -> (Status, Option<Json<Game>>) {
    if let Ok(game) = game::get_game(db, game_id).await {
        (Status::Ok, Some(Json(game)))
    } else {
        (Status::NotFound, None)
    }
}

/// Simulates response of post request to /game by returning game_id of existing game.
///
/// # Arguments
///
/// * `_db` - A MongoDB connection object.
/// * `_game` - A JSON object representing the game to be inserted.
///
/// # Returns
///
/// A tuple containing the HTTP status code 201 and an JSON object with game_id of existing game.
#[post("/gametest", data = "<_game>")]
pub async fn insert_game_test(
    _db: Connection<MongoClient>,
    _game: Json<Game>,
) -> (Status, Option<Json<InsertGameResponse>>) {
    (
        Status::Created,
        Some(Json(InsertGameResponse {
            game_id: "654927c7ade50b1f0b42e1df".to_string(),
        })),
    )
}
