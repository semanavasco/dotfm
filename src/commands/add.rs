use crate::core::error::Error;
use crate::core::repo::Repo;
use std::{os::unix::fs, path::PathBuf};

pub fn add(path: &PathBuf, name: &Option<String>) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    if !path.exists() {
        return Err(Error::Msg("Specified path does not exist.".to_string()));
    }

    let file_name = match name {
        Some(n) => n.clone(),
        None => match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => {
                return Err(Error::Msg(
                    "Could not determine file name from the provided path.".to_string(),
                ));
            }
        },
    };

    if repo.config.files.contains_key(&file_name) {
        return Err(Error::Msg(
            "A file with this name is already managed.".to_string(),
        ));
    }

    repo.config.files.insert(file_name.clone(), path.clone());

    std::fs::rename(path, repo.root().join(&file_name)).map_err(|e| {
        Error::Msg(format!(
            "Failed to move file {} to repository: {}",
            path.display(),
            e
        ))
    })?;
    println!("Moved {} to repository.", path.display());

    fs::symlink(repo.root().join(&file_name), path).map_err(|e| {
        Error::Msg(format!(
            "Failed to create symlink from {} to {}: {}",
            path.display(),
            repo.root().join(&file_name).display(),
            e
        ))
    })?;
    println!(
        "Created symlink from {} to {}",
        path.display(),
        repo.root().join(&file_name).display()
    );

    repo.config.save(repo.config_path())?;
    println!(
        "Added {} to {} repository.",
        path.display(),
        repo.config.name
    );
    Ok(())
}
