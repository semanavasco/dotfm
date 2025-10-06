use crate::core::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub files: HashMap<String, String>,
}

impl Config {
    pub fn new(name: String, author: String) -> Self {
        Config {
            name,
            author,
            files: HashMap::new(),
        }
    }

    pub fn load(file_path: &PathBuf) -> Result<Self, Error> {
        if !file_path.exists() {
            return Err(Error::Msg("Configuration file not found".to_string()));
        }
        let content = std::fs::read_to_string(file_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, file_path: &PathBuf) -> Result<(), Error> {
        let content = toml::to_string(self)?;
        std::fs::write(file_path, content)?;
        Ok(())
    }
}
