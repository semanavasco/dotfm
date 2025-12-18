use std::path::PathBuf;

use crate::GlobalConfig;
use crate::core::{error::Error, repo::Repo};

pub fn remove(repository: Option<PathBuf>, name: String) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let mut repo = Repo::load_at(repo_path)?;

    let packages = repo.config.packages.as_mut().ok_or_else(|| {
        Error::Msg("No managed package manager with this name in the repository.".to_string())
    })?;

    if !packages.contains_key(&name) {
        return Err(Error::Msg(
            "No managed package manager with this name in the repository.".to_string(),
        ));
    }

    packages.remove(&name);

    repo.config.save(repo.config_path())?;
    println!(
        "Removed package manager \"{name}\" from {} repository.",
        repo.config.name
    );
    Ok(())
}
