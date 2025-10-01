use std::path::PathBuf;

use crate::core::config::Config;

pub struct Repo {
    root: PathBuf,
    config_path: PathBuf,
    pub config: Config,
}

impl Repo {
    pub fn new_at(path: PathBuf, force: &bool) -> Result<Self, String> {
        let config_path = path.join(".dotfm");
        if config_path.exists() {
            return Err(String::from("Already in a dotfm repository."));
        }

        let is_empty = match path.read_dir() {
            Ok(mut entries) => entries.next().is_none(),
            Err(_) => return Err(String::from("Failed to read current directory.")),
        };

        if !is_empty && !force {
            return Err(String::from(
                "Directory is not empty. Use --force to initialize anyway.",
            ));
        }

        let config = Config::new(
            match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => String::from("dotfm-repo"),
            },
            String::from("dotfm"),
        );

        Ok(Self {
            root: path,
            config,
            config_path,
        })
    }

    pub fn load_at(path: PathBuf) -> Result<Self, String> {
        let config_path = path.join(".dotfm");
        if !config_path.exists() {
            return Err(String::from("Not in a dotfm repository."));
        }

        let config = match Config::load(&config_path) {
            Ok(cfg) => cfg,
            Err(e) => return Err(format!("Failed to load config: {}", e)),
        };

        Ok(Self {
            root: path,
            config_path,
            config,
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}
