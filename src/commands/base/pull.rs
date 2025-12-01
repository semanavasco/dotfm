use crate::core::error::Error;
use crate::core::paths;
use crate::core::repo::Repo;
use std::path::PathBuf;

pub fn pull(names: &Option<Vec<String>>) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;
    let repo_files = match &repo.config.files {
        Some(files) => files,
        None => {
            return Err(Error::Msg(
                "No files registered in this repository.".to_string(),
            ));
        }
    };

    let files_to_pull: Vec<(String, String)> = match names {
        Some(names_list) => {
            let mut files = Vec::new();
            for name in names_list {
                if let Some(path) = repo_files.get(name) {
                    files.push((name.clone(), path.clone()));
                } else {
                    eprintln!("File '{}' is not managed by dotfm. Skipping it.", name);
                }
            }
            files
        }
        None => repo_files
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    };

    for (name, path_str) in files_to_pull {
        let local_path = PathBuf::from(shellexpand::full(&path_str)?.to_string());
        let repo_path = repo.root().join(&name);

        if !local_path.exists() {
            eprintln!(
                "Local file '{}' does not exist. Skipping.",
                local_path.display()
            );
            continue;
        }

        if local_path.is_symlink() {
            match (
                std::fs::canonicalize(&local_path),
                std::fs::canonicalize(&repo_path),
            ) {
                (Ok(p1), Ok(p2)) if p1 == p2 => {
                    println!(
                        "{} is symlinked to the repository. Already up to date.",
                        name
                    );
                    continue;
                }
                _ => {}
            }
        }

        if repo_path.exists() {
            paths::remove_recursive(&repo_path).map_err(|e| {
                Error::Msg(format!(
                    "Failed to remove existing file in repository at {}: {}",
                    repo_path.display(),
                    e
                ))
            })?;
        }

        paths::copy_recursive(&local_path, &repo_path).map_err(|e| {
            Error::Msg(format!(
                "Failed to copy file from {} to repository: {}",
                local_path.display(),
                e
            ))
        })?;

        println!("Pulled {} from {}", name, local_path.display());
    }
    Ok(())
}
