use crate::core::error::Error;
use crate::core::repo::Repo;
use crate::utils::paths::expand_path;
use std::os::unix::fs;

pub fn load(force: &bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, path_str) in &repo.config.files {
        let path = expand_path(path_str);
        if path.exists() {
            if *force {
                std::fs::remove_file(&path).map_err(|e| {
                    Error::Msg(format!(
                        "Failed to remove existing file {}: {}",
                        path.display(),
                        e
                    ))
                })?;
            } else {
                return Err(Error::Msg(format!(
                    "{} already exists. Use --force to overwrite.",
                    path.display()
                )));
            }
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
