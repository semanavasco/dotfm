use clap::{Parser, Subcommand, ValueHint};

#[derive(Parser, Debug)]
#[command(version, about = "A simple dotfiles manager", long_about = None, author = "svasco")]
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
        /// The file or directory to add.
        #[arg(value_name = "PATH", value_hint = ValueHint::FilePath)]
        path: String,

        /// The name to use for the managed file or directory
        #[arg(short, long)]
        name: Option<String>,

        /// Add file or directory to repository and create symlink to original location
        #[arg(short, long, default_value_t = false)]
        link: bool,
    },

    /// Remove a file or directory from being managed
    Remove {
        /// The file or directory to remove
        #[arg(value_name = "NAME")]
        name: String,

        /// Don't restore file or directory to original location
        #[arg(short, long, default_value_t = false)]
        no_restore: bool,
    },

    /// Push repository to local dotfiles
    Push {
        /// Force overwrite of existing files
        #[arg(short, long, default_value_t = false)]
        force: bool,

        /// Push file or directories as symlinks
        #[arg(short, long, default_value_t = false)]
        link: bool,
    },

    /// Pull local dotfiles to repository
    Pull {
        /// Optional list of dotfiles to pull
        #[arg(value_name = "NAMES")]
        names: Option<Vec<String>>,
    },

    /// Manage package managers and dependencies
    Package {
        #[clap(subcommand)]
        commands: Package,
    },
}

#[derive(Subcommand, Debug)]
pub enum Package {
    /// Add a package
    Add {
        /// The name of the package to add
        name: String,

        /// The package manager to use
        package_manager: String,

        /// Set this package as optional (a description is recommended if package is optional)
        #[arg(short, long, default_value_t = false)]
        optional: bool,
    },

    /// Remove a package
    Remove {
        /// The name of the package to remove
        name: String,

        /// The package manager to remove the package from
        package_manager: String,

        /// Set this package as optional (a description is recommended if package is optional)
        #[arg(short, long, default_value_t = false)]
        optional: bool,
    },

    /// Install packages
    Install {
        /// Optional list of package managers to include
        managers: Option<Vec<String>>,

        /// Install optional packages as well
        #[arg(short, long, default_value_t = false)]
        optional: bool,
    },

    /// Manage package managers
    Manager {
        #[clap(subcommand)]
        commands: PackageManager,
    },
}

#[derive(Subcommand, Debug)]
pub enum PackageManager {
    /// Add a package manager
    Add {
        /// The name of the package manager to add
        name: String,

        /// The install command prefix (ex: sudo pacman -S)
        install_cmd: String,
    },

    /// Remove a package manager
    Remove {
        /// The name of the package manager to remove
        name: String,
    },
}
