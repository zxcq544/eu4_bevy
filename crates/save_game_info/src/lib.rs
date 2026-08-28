use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LastSaveGameInfo {
    pub last_save_game_exists: bool,
    pub save_game_info: SaveGameInfo,
    // pub last_save_game_name: String,
    // pub last_save_game_date: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SaveGameInfo {}
