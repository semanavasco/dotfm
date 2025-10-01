use crate::core::repo::Repo;

pub fn remove(name: &String) -> Result<(), String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to get current working directory.")),
    };

    let mut repo = match Repo::load_at(current_dir) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    if !repo.config.files.contains_key(name) {
        return Err(String::from("No managed file with this name."));
    }

    let original_path = match repo.config.files.get(name) {
        Some(p) => p.clone(),
        None => {
            return Err(String::from(
                "No managed file with this name in the repository.",
            ));
        }
    };

    if original_path.exists() {
        match std::fs::remove_file(&original_path) {
            Ok(_) => {
                println!("Removed managed file: {}", original_path.display());
            }
            Err(e) => {
                return Err(format!(
                    "Failed to remove managed file {}: {}",
                    original_path.display(),
                    e
                ));
            }
        }
    }

    match std::fs::rename(repo.root().join(name), &original_path) {
        Ok(_) => {
            println!(
                "Restored original file from repository to {}",
                original_path.display()
            );
        }
        Err(e) => {
            return Err(format!(
                "Failed to restore original file to {}: {}",
                original_path.display(),
                e
            ));
        }
    };

    repo.config.files.remove(name);

    match repo.config.save(repo.config_path()) {
        Ok(_) => {
            println!("Removed {} from {} repository.", name, repo.config.name);
            Ok(())
        }
        Err(_) => Err(String::from("Couldn't update .dotfm file.")),
    }
}
