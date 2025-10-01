use crate::config::Config;
use std::{env::current_dir, os::unix::fs, path::PathBuf};

pub fn add(path: &PathBuf, name: &Option<String>) -> Result<(), String> {
    let config_file = current_dir().unwrap().join(".dotfm");
    let mut config = Config::load(&config_file).unwrap();

    if !path.exists() {
        return Err(String::from("Specified path does not exist."));
    }

    let file_name = match name {
        Some(n) => n.clone(),
        None => path.file_name().unwrap().to_string_lossy().to_string(),
    };

    if config.files.contains_key(&file_name) {
        return Err(String::from("A file with this name is already managed."));
    }

    config.files.insert(file_name.clone(), path.clone());

    std::fs::rename(path, config_file.parent().unwrap().join(&file_name)).unwrap();
    fs::symlink(config_file.parent().unwrap().join(&file_name), path).unwrap();

    match config.save(&config_file) {
        Ok(_) => {
            println!("Added {} to {} repository.", path.display(), config.name);
            Ok(())
        }
        Err(_) => Err(String::from("Couldn't update .dotfm file.")),
    }
}
