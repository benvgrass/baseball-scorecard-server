use crate::models::team::Team;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Represents a baseball game between two teams.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {

    /// The ID of the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    /// The home team.
    pub home: Team,
    /// The away team.
    pub away: Team,

    /// The date and time of the game.
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        default = "default_time"
    )]
    pub date: chrono::DateTime<chrono::Utc>,
    // pub location: String
}

/// The default time for a game if none is provided.
fn default_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

/// The response returned when a game is inserted into the database.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InsertGameResponse {
    /// The ID of the inserted game.
    pub game_id: String,
}
