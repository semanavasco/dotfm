use crate::config::Config;
use std::env::current_dir;

pub fn init(force: &bool) {
    let current_dir = current_dir().unwrap();

    if current_dir.read_dir().unwrap().next().is_some() && !force {
        eprintln!("Error: Directory is not empty. Use --force to initialize anyway.");
        std::process::exit(1);
    }

    if current_dir.join(".dotfm").exists() {
        eprintln!("Error: Already in a dotfm repository.");
        std::process::exit(1);
    }

    let config = Config::new(
        current_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        String::from("dotfm"),
    );

    match config.save(&current_dir.join(".dotfm")) {
        Ok(_) => println!(
            "Initialized empty dotfm repository in {}",
            current_dir.display()
        ),
        Err(_) => {
            eprintln!(
                "Error: Couldn't write .dotfm file. Check current working directory's permissions."
            );
            std::process::exit(1);
        }
    }
}
