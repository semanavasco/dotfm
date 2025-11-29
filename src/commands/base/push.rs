use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;
use std::os::unix::fs;
use std::path::PathBuf;

pub fn push(force: bool, link: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, path_str) in &repo.config.files {
        let path = PathBuf::from(shellexpand::full(path_str)?.to_string());

        if path.exists() {
            if !force {
                return Err(Error::Msg(format!(
                    "{} already exists. Use --force to overwrite.",
                    path.display()
                )));
            }

            paths::remove_recursive(&path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to remove existing file or directory at {}: {}",
                    path.display(),
                    e
                ))
            })?;
        }

        if link {
            fs::symlink(repo.root().join(name), &path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to create symlink for {}: {}",
                    path.display(),
                    e
                ))
            })?;
        } else {
            paths::copy_recursive(repo.root().join(name).as_path(), &path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to copy file or directory from {} to {} : {}",
                    repo.root().join(name).display(),
                    path.display(),
                    e
                ))
            })?;
        }
        println!("Pushed {} to {}", name, path.display());
    }
    Ok(())
}
