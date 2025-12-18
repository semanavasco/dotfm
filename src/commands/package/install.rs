use std::path::PathBuf;

use crate::GlobalConfig;
use crate::core::{config::PackageManager, error::Error, repo::Repo};

pub fn install(
    repository: Option<PathBuf>,
    managers: &Option<Vec<String>>,
    optional: bool,
) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let repo = Repo::load_at(repo_path)?;

    let packages = repo
        .config
        .packages
        .as_ref()
        .ok_or_else(|| Error::Msg("No packages configured in this repository.".to_string()))?;

    let managers: Vec<(&String, &PackageManager)> = match managers {
        Some(managers) => packages
            .iter()
            .filter(|p| {
                managers.contains(p.0)
                    && (!p.1.dependencies.is_empty() || (optional && !p.1.optional.is_empty()))
            })
            .collect(),
        None => packages
            .iter()
            .filter(|p| !p.1.dependencies.is_empty() || (optional && !p.1.optional.is_empty()))
            .collect(),
    };

    if managers.is_empty() {
        return Err(Error::Msg(
            "No packages to install with current options.".to_string(),
        ));
    }

    for (name, pm) in &managers {
        println!("Installing packages for {}:", name);
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(pm.install_cmd(optional))
            .status()
            .map_err(|e| Error::Msg(format!("Failed to run install command: {}", e)))?;

        if !status.success() {
            return Err(Error::Msg(format!(
                "Install command for {} failed with exit code: {}",
                name,
                status.code().unwrap_or(-1)
            )));
        }
    }

    println!("Installed all packages");
    Ok(())
}
