use crate::core::error::Error;
use crate::core::repo::Repo;
use std::os::unix::fs;
use std::path::PathBuf;

pub fn load(force: &bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, path_str) in &repo.config.files {
        let path = PathBuf::from(shellexpand::full(path_str)?.to_string());

        if path.exists() && !*force {
            if !*force {
                return Err(Error::Msg(format!(
                    "{} already exists. Use --force to overwrite.",
                    path.display()
                )));
            }

            std::fs::remove_file(&path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to remove existing file {}: {}",
                    path.display(),
                    e
                ))
            })?;
        }

        fs::symlink(repo.root().join(name), &path).map_err(|e| {
            Error::Msg(format!(
                "Failed to create symlink for {}: {}",
                path.display(),
                e
            ))
        })?;
        println!("Loaded {} to {}", name, path.display());
    }
    Ok(())
}
