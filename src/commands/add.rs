use crate::core::error::Error;
use crate::core::paths::expand_path;
use crate::core::repo::Repo;
use std::os::unix::fs;

pub fn add(path: &String, name: &Option<String>) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let file_path = expand_path(path);
    if !file_path.exists() {
        return Err(Error::Msg("Specified path does not exist.".to_string()));
    }

    let file_name = match name {
        Some(n) => n.clone(),
        None => match file_path.file_name() {
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

    std::fs::rename(&file_path, repo.root().join(&file_name))
        .map_err(|e| Error::Msg(format!("Failed to move file {} to repository: {}", path, e)))?;
    fs::symlink(repo.root().join(&file_name), &file_path).map_err(|e| {
        Error::Msg(format!(
            "Failed to create symlink from {} to {}: {}",
            path,
            repo.root().join(&file_name).display(),
            e
        ))
    })?;

    repo.config.save(repo.config_path())?;
    println!("Added {} to {} repository.", path, repo.config.name);
    Ok(())
}
