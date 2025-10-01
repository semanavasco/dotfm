use crate::core::config::Config;
use std::env::current_dir;

pub fn init(force: &bool) -> Result<(), String> {
    let current_dir = current_dir().unwrap();

    if current_dir.read_dir().unwrap().next().is_some() && !force {
        return Err(String::from(
            "Directory is not empty. Use --force to initialize anyway.",
        ));
    }

    if current_dir.join(".dotfm").exists() {
        return Err(String::from(
            "A dotfm repository already exists in this directory.",
        ));
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
        Ok(_) => {
            println!(
                "Initialized empty dotfm repository in {}",
                current_dir.display()
            );
            Ok(())
        }
        Err(_) => Err(String::from(
            "Couldn't write .dotfm file. Check current working directory's permissions.",
        )),
    }
}
