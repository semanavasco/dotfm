use crate::config::Config;
use std::{env::current_dir, os::unix::fs};

pub fn load(force: &bool) {
    let current_dir = current_dir().unwrap();

    if !current_dir.join(".dotfm").exists() {
        eprintln!("Error: Not in a dotfm repository.");
        std::process::exit(1);
    }

    let config_content = std::fs::read_to_string(current_dir.join(".dotfm")).unwrap();
    let config: Config = toml::from_str(&config_content).unwrap();

    for (name, original_path) in &config.files {
        if original_path.exists() {
            if *force {
                std::fs::remove_file(original_path).unwrap();
            } else {
                eprintln!(
                    "Error: {} already exists. Use --force to overwrite.",
                    original_path.display()
                );
                continue;
            }
        }

        fs::symlink(current_dir.join(name), original_path).unwrap();
        println!("Loaded {} to {}", name, original_path.display());
    }
}
