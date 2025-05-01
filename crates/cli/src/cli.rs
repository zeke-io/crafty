pub use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "crafty", bin_name = "crafty", author, version, about)]
pub struct Cli {
    /// Profile name
    #[arg(long)]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a project with a basic template
    New { path: PathBuf },
    /// Initialize project in current directory
    Init,
    /// Install the files and plugins for a project
    Install {
        #[arg(short, long)]
        force: bool,
    },
    /*/// Add a plugin to the project
    Add { name: String },*/
    /// Run the server project
    Run {
        #[arg(long)]
        no_setup: bool,
    },
    /// Pack the server and its files
    Pack,
}
