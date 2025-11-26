use crate::core::error::Error;
use crate::core::repo::Repo;
use std::fs;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};

fn copy_recursive(src: &Path, dst: &Path) -> IOResult<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            copy_recursive(&src_path, &dst_path)?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

fn remove_recursive(path: &Path) -> IOResult<()> {
    if path.is_symlink() || path.is_file() {
        fs::remove_file(path)
    } else if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        Ok(())
    }
}

pub fn restore(force: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    for (name, path_str) in &repo.config.files {
        let path = PathBuf::from(shellexpand::full(path_str)?.to_string());

        if path.exists() {
            if force || path.is_symlink() {
                remove_recursive(&path).map_err(|e| {
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
        copy_recursive(&src, &path).map_err(|e| {
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
