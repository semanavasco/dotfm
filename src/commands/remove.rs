use crate::config::Config;
use std::env::current_dir;

pub fn remove(name: &String) {
    let current_dir = current_dir().unwrap();

    if !current_dir.join(".dotfm").exists() {
        eprintln!("Error: Not in a dotfm repository.");
        std::process::exit(1);
    }

    let mut config_content = std::fs::read_to_string(current_dir.join(".dotfm")).unwrap();
    let mut config: Config = toml::from_str(&config_content).unwrap();

    if !config.files.contains_key(name) {
        eprintln!("Error: No managed file with this name.");
        std::process::exit(1);
    }

    let original_path = config.files.get(name).unwrap().clone();
    std::fs::remove_file(&original_path).unwrap();
    std::fs::rename(current_dir.join(name), &original_path).unwrap();

    config.files.remove(name);

    config_content = toml::to_string(&config).unwrap();
    match std::fs::write(current_dir.join(".dotfm"), config_content) {
        Ok(_) => println!("Removed {} from {} repository.", name, config.name),
        Err(_) => {
            eprintln!("Error: Couldn't update .dotfm file.");
            std::process::exit(1);
        }
    }
}
