use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub satoshis: u32,
    pub high_score: u32,
    pub damage_level: u32,
    pub fire_rate_level: u32,
}

impl SaveData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Self {
        let save_path = "game_save.json";
        if Path::new(save_path).exists() {
            fs::read_to_string(save_path)
                .ok()
                .and_then(|contents| serde_json::from_str(&contents).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let save_path = "game_save.json";
        let json = serde_json::to_string_pretty(self)?;
        fs::write(save_path, json)?;
        Ok(())
    }
}
