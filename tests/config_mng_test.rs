#[cfg(test)]
mod tests {
    use super::super::config_manager::{Config, ConfigManager};

    #[test]
    fn test_load_config() {
        let config = ConfigManager::load_config("config/test_config.json");
        assert!(config.max_threads > 0);
    }

    #[test]
    fn test_save_config() {
        let config = Config { max_threads: 4, log_level: "DEBUG".to_string() };
        ConfigManager::save_config("config/test_config.json", &config);
        let loaded_config = ConfigManager::load_config("config/test_config.json");
        assert_eq!(loaded_config.max_threads, 4);
    }
}
