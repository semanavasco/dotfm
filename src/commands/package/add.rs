use crate::core::{error::Error, repo::Repo};

pub fn add(name: &str, package_manager: &str, optional: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let package_manager_ref = repo
        .config
        .packages
        .as_mut()
        .and_then(|p| p.get_mut(package_manager))
        .ok_or_else(|| {
            Error::Msg(format!(
                "Package manager named \"{package_manager}\" does not exist in repository. Try adding it first."
            ))
        })?;

    if package_manager_ref.dependencies.contains(&name.to_string()) {
        return Err(Error::Msg(
            "A package with this name is already managed as a dependency.".to_string(),
        ));
    }
    if package_manager_ref.optional.contains(&name.to_string()) {
        return Err(Error::Msg(
            "A package with this name is already managed as an optional dependency.".to_string(),
        ));
    }

    if optional {
        package_manager_ref.optional.push(name.to_string());
    } else {
        package_manager_ref.dependencies.push(name.to_string());
    }

    repo.config.save(repo.config_path())?;
    println!(
        "Package {name} was added to {package_manager}'s{}dependencies.",
        if optional { " optional " } else { " " }
    );
    Ok(())
}
