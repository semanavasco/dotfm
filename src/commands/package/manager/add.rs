use crate::core::{config::PackageManager, error::Error, repo::Repo};

pub fn add(name: &str, install_cmd: &str) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    if repo.config.packages.contains_key(name) {
        return Err(Error::Msg(
            "A package manager with this name is already managed.".to_string(),
        ));
    }

    repo.config
        .packages
        .insert(name.to_string(), PackageManager::new(install_cmd.to_owned()));

    repo.config.save(repo.config_path())?;
    println!(
        "Added package manager \"{name}\" to {} repository.",
        repo.config.name
    );
    Ok(())
}
