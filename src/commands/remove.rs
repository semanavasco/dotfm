use crate::core::error::Error;
use crate::core::repo::Repo;
use crate::utils::paths::expand_path;

pub fn remove(name: &String) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    if !repo.config.files.contains_key(name) {
        return Err(Error::Msg("No managed file with this name.".to_string()));
    }

    let original_path = match repo.config.files.get(name) {
        Some(p) => expand_path(p),
        None => {
            return Err(Error::Msg(
                "No managed file with this name in the repository.".to_string(),
            ));
        }
    };

    if original_path.exists() {
        std::fs::remove_file(&original_path).map_err(|e| {
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
