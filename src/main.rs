use clap::Parser;
use dotfm::commands;
use dotfm::core::cli::{Cli, Commands, Package, PackageManager};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.commands {
        Commands::Init { force } => commands::base::init(*force),
        Commands::Remove { name, no_restore } => commands::base::remove(name),
        Commands::Sync { force, link } => commands::base::sync(*force),
        Commands::Add { path, name, link } => commands::base::add(path, name.as_deref(), *link),
        Commands::Check => commands::base::check(),

        Commands::Package { commands } => match &commands {
            Package::Add {
                name,
                package_manager,
                optional,
            } => commands::package::add(name, package_manager, *optional),
            Package::Remove {
                name,
                package_manager,
                optional,
            } => commands::package::remove(name, package_manager, *optional),
            Package::Install { managers, optional } => {
                commands::package::install(managers, *optional)
            }

            Package::Manager { commands } => match &commands {
                PackageManager::Add { name, install_cmd } => {
                    commands::package::add_manager(name, install_cmd)
                }
                PackageManager::Remove { name } => commands::package::remove_manager(name),
            },
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
