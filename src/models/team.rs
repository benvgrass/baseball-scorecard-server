use crate::models::player::Player;
use serde::{Deserialize, Serialize};

/// Represents a baseball team with a name and a lineup of players.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    /// The name of the team.
    pub name: String,
    /// The lineup of the team, consisting of 9 batters and a pitcher.
    pub lineup: Lineup,
}

/// Represents a lineup of players, consisting of 9 batters and a pitcher.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lineup {
    /// The array of batters in the lineup.
    pub batters: [Player; 9],
    /// The name of the pitcher in the lineup.
    pub pitcher: String,
}
