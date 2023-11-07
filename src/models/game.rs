use crate::models::team::Team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub home: Team,
    pub away: Team,

    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        default = "default_time"
    )]
    pub date: chrono::DateTime<chrono::Utc>,
    // pub location: String
}

fn default_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InsertGameResponse {
    pub game_id: String,
}
