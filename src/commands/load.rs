use crate::core::repo::Repo;
use std::os::unix::fs;

pub fn load(force: &bool) -> Result<(), String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to get current working directory.")),
    };

    let repo = match Repo::load_at(current_dir) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    for (name, original_path) in &repo.config.files {
        if original_path.exists() {
            if *force {
                match std::fs::remove_file(original_path) {
                    Ok(_) => {
                        println!("Removed existing file: {}", original_path.display());
                    }
                    Err(e) => {
                        eprintln!(
                            "Error: Failed to remove existing file {}: {}",
                            original_path.display(),
                            e
                        );
                        continue;
                    }
                }
            } else {
                eprintln!(
                    "Error: {} already exists. Use --force to overwrite.",
                    original_path.display()
                );
                continue;
            }
        }

        match fs::symlink(repo.root().join(name), original_path) {
            Ok(_) => {
                println!("Loaded {} to {}", name, original_path.display());
            }
            Err(e) => {
                eprintln!(
                    "Failed to create symlink for {}: {}",
                    original_path.display(),
                    e
                );
                continue;
            }
        };
    }

    Ok(())
}
