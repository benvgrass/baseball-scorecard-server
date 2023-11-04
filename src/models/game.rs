use serde::{Serialize, Deserialize};
use crate::models::team::Team;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub home: Team,
    pub away: Team,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: chrono::DateTime<chrono::Utc>,
    
    pub location: String
}