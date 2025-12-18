use std::path::PathBuf;

use crate::GlobalConfig;
use crate::core::{config::PackageManager, error::Error, repo::Repo};

pub fn add(repository: Option<PathBuf>, name: String, install_cmd: String) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let mut repo = Repo::load_at(repo_path)?;

    if repo
        .config
        .packages
        .as_ref()
        .is_some_and(|p| p.contains_key(&name))
    {
        return Err(Error::Msg(
            "A package manager with this name is already managed.".to_string(),
        ));
    }

    repo.config
        .packages
        .get_or_insert_with(std::collections::HashMap::new)
        .insert(
            name.to_string(),
            PackageManager::new(install_cmd.to_owned()),
        );

    repo.config.save(repo.config_path())?;
    println!(
        "Added package manager \"{name}\" to {} repository.",
        repo.config.name
    );
    Ok(())
}
