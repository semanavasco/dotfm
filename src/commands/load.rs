use crate::core::error::Error;
use crate::core::repo::Repo;
use std::os::unix::fs;

pub fn load(force: &bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, original_path) in &repo.config.files {
        if original_path.exists() {
            if *force {
                std::fs::remove_file(original_path).map_err(|e| {
                    Error::Msg(format!(
                        "Failed to remove existing file {}: {}",
                        original_path.display(),
                        e
                    ))
                })?;
                println!("Removed existing file: {}", original_path.display());
            } else {
                return Err(Error::Msg(format!(
                    "{} already exists. Use --force to overwrite.",
                    original_path.display()
                )));
            }
        }

        fs::symlink(repo.root().join(name), original_path).map_err(|e| {
            Error::Msg(format!(
                "Failed to create symlink for {}: {}",
                original_path.display(),
                e
            ))
        })?;
        println!("Loaded {} to {}", name, original_path.display());
    }
    Ok(())
}
