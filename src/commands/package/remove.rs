use crate::core::{error::Error, repo::Repo};

pub fn remove(name: &str, package_manager: &str, optional: &bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let mut repo = Repo::load_at(current_dir)?;

    let Some(package_manager_ref) = repo.config.packages.get_mut(package_manager) else {
        return Err(Error::Msg(format!(
            "Package manager named \"{package_manager}\" does not exist in repository."
        )));
    };

    if *optional {
        if !package_manager_ref.optional.contains(&name.to_string()) {
            return Err(Error::Msg(
                "A package with this name isn't managed as an optional dependency.".to_string(),
            ));
        }

        package_manager_ref.optional.retain(|p| p != name);
    } else {
        if !package_manager_ref.dependencies.contains(&name.to_string()) {
            return Err(Error::Msg(
                "A package with this name isn't managed as a dependency.".to_string(),
            ));
        }

        package_manager_ref.dependencies.retain(|p| p != name);
    }

    repo.config.save(repo.config_path())?;
    println!(
        "Package {name} was removed from {package_manager}'s{}dependencies.",
        if *optional { " optional " } else { " " }
    );
    Ok(())
}
