use serde::{Deserialize, Serialize};

/// Represents a baseball player.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    /// The name of the player.
    pub name: String,
    /// The position of the player.
    pub position: String,
}
