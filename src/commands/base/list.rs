use crate::GlobalConfig;
use crate::core::{error::Error, repo::Repo};
use colored::Colorize;
use std::path::PathBuf;

pub fn list(repository: Option<PathBuf>, no_files: bool, no_packages: bool) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let repo = Repo::load_at(repo_path)?;

    if no_files && no_packages {
        return Err(Error::Msg("You must display something.".to_string()));
    }

    if !no_files && let Some(files) = &repo.config.files {
        println!("{}", "Managed Files:".underline().bold());

        for (name, path) in files {
            println!("  \"{name}\" -> {path}");
        }
    }

    if !no_packages && let Some(packages) = &repo.config.packages {
        println!("{}", "Managed Packages:".underline().bold());

        for (name, config) in packages {
            println!("  {}", format!("{name}:").bold());
            println!("  - Install Command: {}", config.install_cmd);

            if !config.dependencies.is_empty() {
                println!("  - Dependencies:");

                for package in &config.dependencies {
                    println!("    - {package}");
                }
            }

            if !config.optional.is_empty() {
                println!("  - Optional:");

                for package in &config.optional {
                    println!("    - {package}");
                }
            }
        }
    }

    Ok(())
}
