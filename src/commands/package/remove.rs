use std::path::PathBuf;

use crate::GlobalConfig;
use crate::core::{error::Error, repo::Repo};

pub fn remove(
    repository: Option<PathBuf>,
    name: String,
    package_manager: String,
    optional: bool,
) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let mut repo = Repo::load_at(repo_path)?;

    let package_manager_ref = repo
        .config
        .packages
        .as_mut()
        .and_then(|p| p.get_mut(&package_manager))
        .ok_or_else(|| {
            Error::Msg(format!(
                "Package manager named \"{package_manager}\" does not exist in repository."
            ))
        })?;

    if optional {
        if !package_manager_ref.optional.contains(&name.to_string()) {
            return Err(Error::Msg(
                "A package with this name isn't managed as an optional dependency.".to_string(),
            ));
        }

        package_manager_ref.optional.retain(|p| p != &name);
    } else {
        if !package_manager_ref.dependencies.contains(&name.to_string()) {
            return Err(Error::Msg(
                "A package with this name isn't managed as a dependency.".to_string(),
            ));
        }

        package_manager_ref.dependencies.retain(|p| p != &name);
    }

    repo.config.save(repo.config_path())?;
    println!(
        "Package {name} was removed from {package_manager}'s{}dependencies.",
        if optional { " optional " } else { " " }
    );
    Ok(())
}
