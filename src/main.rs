use clap::Parser;
use dotfm::commands;
use dotfm::core::cli::{Cli, Commands, Package, PackageManager};

fn main() {
    let cli = Cli::parse();

    let result = match cli.commands {
        Commands::Init { force } => commands::base::init(force),
        Commands::Add {
            path,
            name,
            link,
            repository,
        } => commands::base::add(repository, path, name, link),
        Commands::Remove {
            name,
            no_restore,
            repository,
        } => commands::base::remove(repository, name, no_restore),
        Commands::Push {
            force,
            link,
            repository,
        } => commands::base::push(repository, force, link),
        Commands::Pull { names, repository } => commands::base::pull(repository, names),
        Commands::Diff { name, repository } => commands::base::diff(repository, name),
        Commands::List {
            no_files,
            no_packages,
            repository,
        } => commands::base::list(repository, no_files, no_packages),

        Commands::Package { commands } => match commands {
            Package::Add {
                name,
                package_manager,
                optional,
                repository,
            } => commands::package::add(repository, name, package_manager, optional),
            Package::Remove {
                name,
                package_manager,
                optional,
                repository,
            } => commands::package::remove(repository, name, package_manager, optional),
            Package::Install {
                managers,
                optional,
                repository,
            } => commands::package::install(repository, managers, optional),

            Package::Manager { commands } => match commands {
                PackageManager::Add {
                    name,
                    install_cmd,
                    repository,
                } => commands::package::add_manager(repository, name, install_cmd),
                PackageManager::Remove { name, repository } => {
                    commands::package::remove_manager(repository, name)
                }
            },
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
