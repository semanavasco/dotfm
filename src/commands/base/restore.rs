use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;
use std::path::PathBuf;

pub fn restore(force: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, path_str) in &repo.config.files {
        let path = PathBuf::from(shellexpand::full(path_str)?.to_string());

        if path.exists() {
            if force || path.is_symlink() {
                paths::remove_recursive(&path).map_err(|e| {
                    Error::Msg(format!(
                        "Failed to remove existing file or directory at {}: {}",
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

        let src = repo.root().join(name);
        paths::copy_recursive(&src, &path).map_err(|e| {
            Error::Msg(format!(
                "Failed to copy {} to {}: {}",
                src.display(),
                path.display(),
                e
            ))
        })?;

        println!("Restored {} to {}", name, path.display());
    }
    Ok(())
}
