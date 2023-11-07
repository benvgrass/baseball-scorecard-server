use crate::{
    db::{game, MongoClient},
    models::game::{Game, InsertGameResponse},
};
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;

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
