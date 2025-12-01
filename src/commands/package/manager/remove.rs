use crate::core::{error::Error, repo::Repo};

pub fn remove(name: &str) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let packages = repo.config.packages.as_mut().ok_or_else(|| {
        Error::Msg("No managed package manager with this name in the repository.".to_string())
    })?;

    if !packages.contains_key(name) {
        return Err(Error::Msg(
            "No managed package manager with this name in the repository.".to_string(),
        ));
    }

    packages.remove(name);

    repo.config.save(repo.config_path())?;
    println!(
        "Removed package manager \"{name}\" from {} repository.",
        repo.config.name
    );
    Ok(())
}
