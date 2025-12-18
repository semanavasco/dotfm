use std::path::PathBuf;

use crate::core::error::Error;

pub mod commands;
pub mod core;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GlobalConfig {
    pub repository: PathBuf,
}

impl GlobalConfig {
    pub fn get_repository_path(repository: Option<PathBuf>) -> Result<PathBuf, Error> {
        if let Some(repository) = repository {
            return Ok(repository);
        }

        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("dotfm").join("dotfm.toml");

            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                let config: GlobalConfig = toml::from_str(&content)?;

                let path_str = config.repository.to_string_lossy();
                let path = shellexpand::full(&path_str)?;
                return Ok(PathBuf::from(path.to_string()));
            }
        }

        Ok(std::env::current_dir()?)
    }
}
