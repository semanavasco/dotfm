use std::path::PathBuf;

use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;

pub fn remove(name: &str, no_restore: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let Some(path_str) = repo.config.files.as_ref().and_then(|f| f.get(name)) else {
        return Err(Error::Msg("No managed file with this name.".to_string()));
    };

    let original_path = PathBuf::from(shellexpand::full(path_str)?.to_string());
    let repo_path = repo.root().join(name);

    let is_link_to_repo = if original_path.is_symlink() {
        match (
            std::fs::canonicalize(&original_path),
            std::fs::canonicalize(&repo_path),
        ) {
            (Ok(p1), Ok(p2)) => p1 == p2,
            _ => false,
        }
    } else {
        false
    };

    if no_restore {
        if is_link_to_repo {
            paths::remove_recursive(&original_path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to remove symlink at {}: {}",
                    original_path.display(),
                    e
                ))
            })?;
        }
    } else {
        if original_path.exists() || original_path.is_symlink() {
            paths::remove_recursive(&original_path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to remove existing file or directory at {}: {}",
                    original_path.display(),
                    e
                ))
            })?;
        }

        paths::copy_recursive(&repo_path, &original_path)?;
        println!("Restored {} from repository.", original_path.display());
    }

    paths::remove_recursive(&repo_path).map_err(|e| {
        Error::Msg(format!(
            "Failed to remove previously managed file or directory at {}: {}",
            repo_path.display(),
            e
        ))
    })?;

    if let Some(files) = repo.config.files.as_mut() {
        files.remove(name);
    }
    repo.config.save(repo.config_path())?;
    println!("Removed {} from {} repository.", name, repo.config.name);
    Ok(())
}
