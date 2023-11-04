use crate::models::player::Player;
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    pub name: String,
    pub lineup: Lineup,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lineup {
    pub batters: [Player; 9],
    pub pitcher: Player,
}