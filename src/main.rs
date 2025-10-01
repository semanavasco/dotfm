mod commands;
mod core;

use crate::commands::{add, init, load, remove};
use crate::core::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.commands {
        Commands::Init { force } => init::init(force),
        Commands::Add { path, name } => add::add(path, name),
        Commands::Remove { name } => remove::remove(name),
        Commands::Load { force } => load::load(force),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
