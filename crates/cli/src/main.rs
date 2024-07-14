mod cli;
mod runtime;

use anyhow::Context;

use cli::{Cli, Commands, Parser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let current_directory = std::env::current_dir()?;

    // Initialize logger
    if std::env::var_os("CRAFTY_LOG").is_none() {
        std::env::set_var("CRAFTY_LOG", "info")
    }
    pretty_env_logger::init_custom_env("CRAFTY_LOG");

    // Load .env files
    dotenv_flow::dotenv_flow().ok();

    let profile_name: Option<String> = match (cli.profile, &cli.command) {
        // No matter the command, set the profile if provided
        (Some(profile), _) => Some(profile),
        // Set the profile as "dev" if no profile name is provided and the command is `run`
        (None, Commands::Run { .. }) => Some("dev".into()),
        // For any other command, if the profile is not provided, it will be None
        (None, _) => None,
    };

    if let Some(profile_name) = &profile_name {
        log::info!("Using profile: {}", profile_name);
        std::env::set_var("DOTENV_ENV", profile_name);
        std::env::set_var("CRAFTY_PROFILE", profile_name);
    }

    match cli.command {
        Commands::New { path } => templater::generate_template(current_directory.join(path))
            .context("Generating template"),
        Commands::Init => templater::generate_template(current_directory),
        Commands::Install { force } => project::install(current_directory, force).await,
        // Commands::Add { name } => project::add_dependency(current_directory, name).await,
        Commands::Run { no_setup } => {
            runtime::run_project(current_directory, profile_name, no_setup).await
        }
        Commands::Pack => project::packager::pack_server(current_directory, profile_name),
    }
}
