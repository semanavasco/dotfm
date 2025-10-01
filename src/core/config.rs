use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub files: HashMap<String, PathBuf>,
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
            return Err(Error::new(
                ErrorKind::NotFound,
                "Configuration file not found",
            ));
        }

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };
        toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn save(&self, file_path: &PathBuf) -> Result<(), Error> {
        let content = match toml::to_string(self) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };
        std::fs::write(file_path, content)
    }
}
