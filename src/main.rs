use clap::Parser;
use dotfm::commands;
use dotfm::core::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.commands {
        Commands::Init { force } => commands::base::init(force),
        Commands::Add { path, name } => commands::base::add(path, name),
        Commands::Remove { name } => commands::base::remove(name),
        Commands::Load { force } => commands::base::load(force),
        Commands::Restore { force } => commands::base::restore(force),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
