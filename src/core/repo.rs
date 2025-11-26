use crate::core::config::Config;
use crate::core::error::Error;
use std::path::PathBuf;

pub struct Repo {
    root: PathBuf,
    config_path: PathBuf,
    pub config: Config,
}

impl Repo {
    pub fn new_at(path: PathBuf, force: &bool) -> Result<Self, Error> {
        let config_path = path.join("dotfm.toml");
        if config_path.exists() {
            return Err(Error::Msg("Already in a dotfm repository.".to_string()));
        }

        let is_empty = match path.read_dir() {
            Ok(mut entries) => entries.next().is_none(),
            Err(_) => {
                return Err(Error::Msg("Failed to read current directory.".to_string()));
            }
        };

        if !is_empty && !force {
            return Err(Error::Msg(
                "Directory is not empty. Use --force to initialize anyway.".to_string(),
            ));
        }

        let config = Config::new(
            match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => "dotfm-repo".to_string(),
            },
            "dotfm".to_string(),
        );

        Ok(Self {
            root: path,
            config,
            config_path,
        })
    }

    pub fn load_at(path: PathBuf) -> Result<Self, Error> {
        let config_path = path.join("dotfm.toml");
        if !config_path.exists() {
            return Err(Error::Msg("Not in a dotfm repository.".to_string()));
        }

        let config = match Config::load(&config_path) {
            Ok(cfg) => cfg,
            Err(e) => return Err(Error::Msg(format!("Failed to load config: {:?}", e))),
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
