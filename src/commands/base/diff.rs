use colored::Colorize;
use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::GlobalConfig;
use crate::core::{diffs, error::Error, repo::Repo};

pub fn diff(repository: Option<PathBuf>, name: &str) -> Result<(), Error> {
    let repo_path = GlobalConfig::get_repository_path(repository)?;
    let repo = Repo::load_at(repo_path)?;

    let Some(path_str) = repo.config.files.as_ref().and_then(|f| f.get(name)) else {
        return Err(Error::Msg(format!("'{}' is not a managed file.", name)));
    };

    let local_path = PathBuf::from(shellexpand::full(path_str)?.to_string());

    if local_path.is_symlink() {
        return Err(Error::Msg(format!(
            "'{}' is a symlink. Cannot diff symlinks.",
            local_path.display()
        )));
    }

    let repo_path = repo.root().join(name);

    if local_path.is_dir() && repo_path.is_dir() {
        diff_directories(&repo_path, &local_path)?;
    } else if local_path.is_file() && repo_path.is_file() {
        diffs::print_file_header(&repo_path, &local_path);
        if !diffs::print_diffs(&repo_path, &local_path)? {
            diffs::print_no_diffs();
        }
    } else if repo_path.exists() && !local_path.exists() {
        println!(
            "{} '{}' exists in repo but not locally.",
            "Warning:".yellow().bold(),
            name
        );
    } else {
        return Err(Error::Msg(format!(
            "Type mismatch: repo has {}, local has {}.",
            if repo_path.is_dir() {
                "directory"
            } else {
                "file"
            },
            if local_path.is_dir() {
                "directory"
            } else {
                "file"
            }
        )));
    }

    Ok(())
}

fn diff_directories(repo_path: &PathBuf, local_path: &PathBuf) -> Result<(), Error> {
    let repo_files: HashSet<PathBuf> = WalkDir::new(repo_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.path().strip_prefix(repo_path).ok().map(PathBuf::from))
        .collect();

    let local_files: HashSet<PathBuf> = WalkDir::new(local_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.path().strip_prefix(local_path).ok().map(PathBuf::from))
        .collect();

    let only_in_repo: Vec<&PathBuf> = repo_files.difference(&local_files).collect();
    let only_in_local: Vec<&PathBuf> = local_files.difference(&repo_files).collect();
    let in_both: Vec<&PathBuf> = repo_files.intersection(&local_files).collect();

    if !only_in_repo.is_empty() {
        println!("\n{}", "Files only in repo:".yellow().bold());
        for file in &only_in_repo {
            diffs::print_only_in_repo(file);
        }
    }

    if !only_in_local.is_empty() {
        println!("\n{}", "Files only locally:".yellow().bold());
        for file in &only_in_local {
            diffs::print_only_in_local(file);
        }
    }

    let mut changed_count = 0;
    for relative_path in &in_both {
        let repo_file = repo_path.join(relative_path);
        let local_file = local_path.join(relative_path);

        diffs::print_file_header(&repo_file, &local_file);
        if diffs::print_diffs(&repo_file, &local_file)? {
            changed_count += 1;
        } else {
            diffs::print_no_diffs();
        }
    }

    println!(
        "\n{} {} file(s) compared, {} with changes, {} only in repo, {} only locally.",
        "Summary:".bold().underline(),
        in_both.len().to_string().bold(),
        changed_count.to_string().bold(),
        only_in_repo.len().to_string().bold(),
        only_in_local.len().to_string().bold()
    );

    Ok(())
}
