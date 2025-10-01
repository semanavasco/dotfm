use crate::core::repo::Repo;
use std::{os::unix::fs, path::PathBuf};

pub fn add(path: &PathBuf, name: &Option<String>) -> Result<(), String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to get current working directory.")),
    };

    let mut repo = match Repo::load_at(current_dir) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    if !path.exists() {
        return Err(String::from("Specified path does not exist."));
    }

    let file_name = match name {
        Some(n) => n.clone(),
        None => match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => {
                return Err(String::from(
                    "Could not determine file name from the provided path.",
                ));
            }
        },
    };

    if repo.config.files.contains_key(&file_name) {
        return Err(String::from("A file with this name is already managed."));
    }

    repo.config.files.insert(file_name.clone(), path.clone());

    match std::fs::rename(path, repo.root().join(&file_name)) {
        Ok(_) => {
            println!("Moved {} to repository.", path.display());
        }
        Err(e) => {
            return Err(format!(
                "Failed to move file {} to repository: {}",
                path.display(),
                e
            ));
        }
    };

    match fs::symlink(repo.root().join(&file_name), path) {
        Ok(_) => {
            println!(
                "Created symlink from {} to {}",
                path.display(),
                repo.root().join(&file_name).display()
            );
        }
        Err(e) => {
            return Err(format!(
                "Failed to create symlink from {} to {}: {}",
                path.display(),
                repo.root().join(&file_name).display(),
                e
            ));
        }
    };

    match repo.config.save(repo.config_path()) {
        Ok(_) => {
            println!(
                "Added {} to {} repository.",
                path.display(),
                repo.config.name
            );
            Ok(())
        }
        Err(_) => Err(String::from("Couldn't update .dotfm file.")),
    }
}
