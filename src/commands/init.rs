use crate::core::repo::Repo;

pub fn init(force: &bool) -> Result<(), String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Failed to get current working directory.")),
    };

    let repo = match Repo::new_at(current_dir.clone(), force) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    match repo.config.save(repo.config_path()) {
        Ok(_) => {
            println!(
                "Initialized empty dotfm repository at {}",
                repo.root().display()
            );
            Ok(())
        }
        Err(_) => Err(String::from(
            "Couldn't write .dotfm file. Check current working directory's permissions.",
        )),
    }
}
