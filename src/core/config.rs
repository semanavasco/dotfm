use crate::core::error::Error;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub files: HashMap<String, String>,
    pub packages: HashMap<String, PackageManager>,
}

impl Config {
    pub fn new(name: String, author: String) -> Self {
        Config {
            name,
            author,
            files: HashMap::new(),
            packages: HashMap::new(),
        }
    }

    pub fn load(file_path: &Path) -> Result<Self, Error> {
        if !file_path.exists() {
            return Err(Error::Msg("Configuration file not found".to_string()));
        }
        let content = std::fs::read_to_string(file_path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, file_path: &Path) -> Result<(), Error> {
        let content = toml::to_string(self)?;
        std::fs::write(file_path, content)?;
        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PackageManager {
    pub install_cmd: String,
    pub dependencies: Vec<String>,
    pub optional: Vec<String>,
}

impl PackageManager {
    pub fn new(install_cmd: String) -> Self {
        PackageManager {
            install_cmd,
            dependencies: Vec::new(),
            optional: Vec::new(),
        }
    }

    pub fn install_cmd(&self, optional: bool) -> String {
        if !optional {
            format!("{} {}", self.install_cmd, self.dependencies.join(" "))
        } else {
            format!(
                "{} {} {}",
                self.install_cmd,
                self.dependencies.join(" "),
                self.optional.join(" ")
            )
        }
    }
}
