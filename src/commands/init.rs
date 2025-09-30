use crate::config::Config;
use std::{collections::HashMap, env::current_dir};

pub fn init(force: &bool) {
    let current_dir = current_dir().unwrap();

    if current_dir.read_dir().unwrap().next().is_some() && !force {
        eprintln!("Error: Directory is not empty. Use --force to initialize anyway.");
        std::process::exit(1);
    }

    if current_dir.join(".dotfm").exists() {
        eprintln!("Error: Already in a dotfm repository.");
        std::process::exit(1);
    }

    let config = Config {
        name: current_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        author: String::from("dotfm"),
        files: HashMap::new(),
    };

    let config_toml = toml::to_string(&config).unwrap();

    match std::fs::write(current_dir.join(".dotfm"), config_toml) {
        Ok(_) => println!(
            "Initialized empty dotfm repository in {}",
            current_dir.display()
        ),
        Err(_) => {
            eprintln!(
                "Error: Couldn't write .dotfm file. Check current working directory's permissions."
            );
            std::process::exit(1);
        }
    }
}
