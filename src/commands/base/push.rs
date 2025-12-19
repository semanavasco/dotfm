use crate::GlobalConfig;
use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;
use std::os::unix::fs;
use std::path::PathBuf;

pub fn push(
    repository: Option<PathBuf>,
    names: Option<Vec<String>>,
    force: bool,
    link: bool,
) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let repo = Repo::load_at(repo_path)?;
    let repo_files = match &repo.config.files {
        Some(files) => files,
        None => {
            return Err(Error::Msg(
                "No files registered in this repository.".to_string(),
            ));
        }
    };

    let files_to_push: Vec<(String, String)> = match names {
        Some(names_list) => {
            let mut files = Vec::new();
            for name in names_list {
                if let Some(path) = repo_files.get(&name) {
                    files.push((name, path.clone()));
                } else {
                    eprintln!("Dotfile '{}' is not managed by dotfm. Skipping it.", name);
                }
            }
            files
        }
        None => repo_files
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    };

    for (name, path_str) in files_to_push {
        let path = PathBuf::from(shellexpand::full(&path_str)?.to_string());

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
            fs::symlink(repo.root().join(&name), &path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to create symlink for {}: {}",
                    path.display(),
                    e
                ))
            })?;
        } else {
            paths::copy_recursive(repo.root().join(&name).as_path(), &path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to copy file or directory from {} to {} : {}",
                    repo.root().join(&name).display(),
                    path.display(),
                    e
                ))
            })?;
        }
        println!("Pushed {} to {}", &name, path.display());
    }
    Ok(())
}
