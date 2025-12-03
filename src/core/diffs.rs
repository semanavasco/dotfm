use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::{fs, path::Path};

use crate::core::error::Error;

pub fn print_diffs(left: &Path, right: &Path) -> Result<bool, Error> {
    let left_content = fs::read_to_string(left)?;
    let right_content = fs::read_to_string(right)?;

    let diff = TextDiff::from_lines(&right_content, &left_content);

    if diff.ratio() == 1.0 {
        return Ok(false);
    }

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("  {}", format!("- {}", change).red()),
            ChangeTag::Insert => print!("  {}", format!("+ {}", change).green()),
            ChangeTag::Equal => print!("  {}", format!("  {}", change).dimmed()),
        };
    }

    Ok(true)
}

pub fn print_file_header(repo_path: &Path, local_path: &Path) {
    println!(
        "\n{} {} {} {}",
        repo_path.display().to_string().cyan(),
        "<=>".bold(),
        local_path.display().to_string().cyan(),
        ":".bold()
    );
}

pub fn print_only_in_repo(path: &Path) {
    println!(
        "  {} {}",
        "+".green().bold(),
        path.display().to_string().green(),
    );
}

pub fn print_only_in_local(path: &Path) {
    println!(
        "  {} {}",
        "-".red().bold(),
        path.display().to_string().red(),
    );
}

pub fn print_no_diffs() {
    println!("{}", "  No differences".dimmed());
}
