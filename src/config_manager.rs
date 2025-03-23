use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub max_threads: usize,
    pub log_level: String,
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load_config(file_path: &str) -> Config {
        let contents = fs::read_to_string(file_path).expect("Failed to read config file");
        serde_json::from_str(&contents).expect("Invalid JSON format")
    }

    pub fn save_config(file_path: &str, config: &Config) {
        let json_string = serde_json::to_string_pretty(config).unwrap();
        fs::write(file_path, json_string).expect("Failed to write config file");
    }
}
