use clap::{Parser, Subcommand, ValueHint};
use std::{collections::HashMap, env::current_dir, os::unix::fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "A dotfiles manager", long_about = None, author = "svasco")]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new dotfiles repository
    Init {
        /// Force initialization even if the directory is not empty
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// Add a new file or directory to be managed
    Add {
        /// The file or directory to add
        #[arg(value_name = "PATH", value_hint = ValueHint::FilePath)]
        path: PathBuf,

        /// The name to use for the managed file or directory
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Remove a file or directory from being managed
    Remove {
        /// The file or directory to remove
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Load the managed files into their respective locations
    Load {
        /// Force overwrite of existing files
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    name: String,
    author: String,
    files: HashMap<String, PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Init { force } => {
            let current_dir = current_dir().unwrap();

            if current_dir.read_dir().unwrap().next().is_some() && !force {
                eprintln!("Error: Directory is not empty. Use --force to initialize anyway.");
                std::process::exit(1);
            }

            if current_dir.join(".dotfm").exists() {
                eprintln!("Error: Already in a dotfm repository.");
                std::process::exit(1);
            }

            let config = Config {
                name: current_dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                author: String::from("dotfm"),
                files: HashMap::new(),
            };

            let config_toml = toml::to_string(&config).unwrap();

            match std::fs::write(current_dir.join(".dotfm"), config_toml) {
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
        Commands::Add { path, name } => {
            let current_dir = current_dir().unwrap();

            if !current_dir.join(".dotfm").exists() {
                eprintln!("Error: Not in a dotfm repository.");
                std::process::exit(1);
            }

            if !path.exists() {
                eprintln!("Error: Specified path does not exist.");
                std::process::exit(1);
            }

            let mut config_content = std::fs::read_to_string(current_dir.join(".dotfm")).unwrap();
            let mut config: Config = toml::from_str(&config_content).unwrap();

            let file_name = match name {
                Some(n) => n.clone(),
                None => path.file_name().unwrap().to_string_lossy().to_string(),
            };

            if config.files.contains_key(&file_name) {
                eprintln!("Error: A file with this name is already managed.");
                std::process::exit(1);
            }

            config.files.insert(file_name.clone(), path.clone());

            std::fs::rename(path, current_dir.join(&file_name)).unwrap();
            fs::symlink(current_dir.join(&file_name), path).unwrap();

            config_content = toml::to_string(&config).unwrap();
            match std::fs::write(current_dir.join(".dotfm"), config_content) {
                Ok(_) => println!("Added {} to {} repository.", path.display(), config.name),
                Err(_) => {
                    eprintln!("Error: Couldn't update .dotfm file.");
                    std::process::exit(1);
                }
            }
        }
        Commands::Remove { name } => {
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
        Commands::Load { force } => {
            let current_dir = current_dir().unwrap();

            if !current_dir.join(".dotfm").exists() {
                eprintln!("Error: Not in a dotfm repository.");
                std::process::exit(1);
            }

            let config_content = std::fs::read_to_string(current_dir.join(".dotfm")).unwrap();
            let config: Config = toml::from_str(&config_content).unwrap();

            for (name, original_path) in &config.files {
                if original_path.exists() {
                    if *force {
                        std::fs::remove_file(original_path).unwrap();
                    } else {
                        eprintln!(
                            "Error: {} already exists. Use --force to overwrite.",
                            original_path.display()
                        );
                        continue;
                    }
                }

                fs::symlink(current_dir.join(name), original_path).unwrap();
                println!("Loaded {} to {}", name, original_path.display());
            }
        }
    }
}
