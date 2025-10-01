use crate::config::Config;
use std::env::current_dir;

pub fn remove(name: &String) -> Result<(), String> {
    let config_file = current_dir().unwrap().join(".dotfm");
    let mut config = Config::load(&config_file).unwrap();

    if !config.files.contains_key(name) {
        return Err(String::from("No managed file with this name."));
    }

    let original_path = config.files.get(name).unwrap().clone();
    if original_path.exists() {
        std::fs::remove_file(&original_path).unwrap();
    }

    std::fs::rename(config_file.parent().unwrap().join(name), &original_path).unwrap();

    config.files.remove(name);

    match config.save(&config_file) {
        Ok(_) => {
            println!("Removed {} from {} repository.", name, config.name);
            Ok(())
        }
        Err(_) => Err(String::from("Couldn't update .dotfm file.")),
    }
}
