use crate::core::error::Error;
use crate::core::repo::Repo;
use std::fs;
use std::path::PathBuf;

pub fn check() -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::load_at(current_dir)?;

    let mut synced = 0;
    let mut unsynced = 0;

    for (name, path_str) in &repo.config.files {
        let path = PathBuf::from(shellexpand::full(path_str)?.to_string());

        if !path.exists() {
            eprintln!("{name}: not synced (target {path_str} doesn't exist)");
            unsynced += 1;
        } else if !path.is_symlink() {
            eprintln!("{name}: not synced ({path_str} exists but isn't a symlink)");
            unsynced += 1;
        } else {
            let sym_path = fs::read_link(&path)?;
            let expected_path = repo.root().join(name);

            let sym_canonical = path.parent().unwrap_or(&path).join(&sym_path);
            let sym_canonical = sym_canonical.canonicalize().ok();
            let expected_canonical = expected_path.canonicalize().ok();

            if sym_canonical.is_some() && sym_canonical == expected_canonical {
                println!("{name}: synced");
                synced += 1;
            } else {
                eprintln!("{name}: not synced (symlink points elsewhere)",);
                unsynced += 1;
            }
        }
    }

    println!(
        "\nChecked {} files: {synced} synced, {unsynced} not synced",
        repo.config.files.len()
    );
    Ok(())
}
