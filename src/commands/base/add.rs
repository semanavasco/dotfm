use crate::GlobalConfig;
use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;
use std::collections::HashMap;
use std::os::unix::fs;
use std::path::PathBuf;

pub fn add(
    repository: Option<PathBuf>,
    path: &str,
    name: Option<&str>,
    link: bool,
) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let mut repo = Repo::load_at(repo_path)?;

    let file_path = PathBuf::from(shellexpand::full(path)?.to_string());

    if !file_path.exists() {
        return Err(Error::Msg("Specified path does not exist.".to_string()));
    }

    let file_name = match name {
        Some(n) => n.to_string(),
        None => match file_path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => {
                return Err(Error::Msg(
                    "Could not determine file name from the provided path.".to_string(),
                ));
            }
        },
    };

    if repo
        .config
        .files
        .as_ref()
        .is_some_and(|f| f.contains_key(&file_name))
    {
        return Err(Error::Msg(
            "A file with this name is already managed.".to_string(),
        ));
    }

    repo.config
        .files
        .get_or_insert_with(HashMap::new)
        .insert(file_name.clone(), path.to_string());

    if link {
        std::fs::rename(&file_path, repo.root().join(&file_name)).map_err(|e| {
            Error::Msg(format!("Failed to move file {} to repository: {}", path, e))
        })?;

        fs::symlink(repo.root().join(&file_name), &file_path).map_err(|e| {
            Error::Msg(format!(
                "Failed to create symlink from {} to {}: {}",
                path,
                repo.root().join(&file_name).display(),
                e
            ))
        })?;
    } else {
        paths::copy_recursive(&file_path, repo.root().join(&file_name).as_path())?;
    }

    repo.config.save(repo.config_path())?;
    println!("Added {} to {} repository.", path, repo.config.name);
    Ok(())
}
