use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub model: String,
    pub system_prompt: String,
}

impl Config {
    pub fn load(config_file: &str) -> Result<Self, String> {
        let config_content = std::fs::read_to_string(config_file)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: Config = serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_config() {
        let config = Config::load("config-example.json");
        assert!(config.is_ok(), "Config should load successfully");
        let config = config.unwrap();
        assert!(!config.server_address.is_empty(), "Server address should not be empty");
        assert!(!config.model.is_empty(), "Model should not be empty");
        assert!(!config.system_prompt.is_empty(), "System prompt should not be empty");
    }
}
