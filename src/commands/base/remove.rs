use std::path::PathBuf;

use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;

pub fn remove(name: &str) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let Some(path_str) = repo.config.files.get(name) else {
        return Err(Error::Msg("No managed file with this name.".to_string()));
    };

    let original_path = PathBuf::from(shellexpand::full(path_str)?.to_string());

    if original_path.exists() {
        paths::remove_recursive(&original_path).map_err(|e| {
            Error::Msg(format!(
                "Failed to remove managed file {}: {}",
                original_path.display(),
                e
            ))
        })?;
    }

    std::fs::rename(repo.root().join(name), &original_path).map_err(|e| {
        Error::Msg(format!(
            "Failed to restore original file to {}: {}",
            original_path.display(),
            e
        ))
    })?;

    repo.config.files.remove(name);
    repo.config.save(repo.config_path())?;
    println!("Removed {} from {} repository.", name, repo.config.name);
    Ok(())
}
