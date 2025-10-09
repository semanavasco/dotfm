use clap::{Parser, Subcommand, ValueHint};

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
        /// The file or directory to add.
        #[arg(value_name = "PATH", value_hint = ValueHint::FilePath)]
        path: String,

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

    /// Restore all managed files to their original locations
    Restore {
        /// Force overwrite of existing files
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}
