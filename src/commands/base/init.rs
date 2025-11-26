use crate::core::error::Error;
use crate::core::repo::Repo;

pub fn init(force: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let repo = Repo::new_at(current_dir, force)?;

    repo.config.save(repo.config_path())?;
    println!(
        "Initialized empty dotfm repository at {}",
        repo.root().display()
    );
    Ok(())
}
