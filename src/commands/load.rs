use crate::core::config::Config;
use std::{env::current_dir, os::unix::fs};

pub fn load(force: &bool) -> Result<(), String> {
    let config_file = current_dir().unwrap().join(".dotfm");
    let config = Config::load(&config_file).unwrap();

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

        fs::symlink(config_file.parent().unwrap().join(name), original_path).unwrap();
        println!("Loaded {} to {}", name, original_path.display());
    }

    Ok(())
}
