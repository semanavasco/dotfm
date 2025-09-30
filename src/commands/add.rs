use crate::config::Config;
use std::{env::current_dir, os::unix::fs, path::PathBuf};

pub fn add(path: &PathBuf, name: &Option<String>) {
    let current_dir = current_dir().unwrap();

    if !current_dir.join(".dotfm").exists() {
        eprintln!("Error: Not in a dotfm repository.");
        std::process::exit(1);
    }

    if !path.exists() {
        eprintln!("Error: Specified path does not exist.");
        std::process::exit(1);
    }

    let mut config_content = std::fs::read_to_string(current_dir.join(".dotfm")).unwrap();
    let mut config: Config = toml::from_str(&config_content).unwrap();

    let file_name = match name {
        Some(n) => n.clone(),
        None => path.file_name().unwrap().to_string_lossy().to_string(),
    };

    if config.files.contains_key(&file_name) {
        eprintln!("Error: A file with this name is already managed.");
        std::process::exit(1);
    }

    config.files.insert(file_name.clone(), path.clone());

    std::fs::rename(path, current_dir.join(&file_name)).unwrap();
    fs::symlink(current_dir.join(&file_name), path).unwrap();

    config_content = toml::to_string(&config).unwrap();
    match std::fs::write(current_dir.join(".dotfm"), config_content) {
        Ok(_) => println!("Added {} to {} repository.", path.display(), config.name),
        Err(_) => {
            eprintln!("Error: Couldn't update .dotfm file.");
            std::process::exit(1);
        }
    }
}
