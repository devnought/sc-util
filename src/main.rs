use std::path::PathBuf;

use clap::{Parser, Subcommand};
use sc_util::*;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Args {
    /// Clear user folder or shaders
    Clean {
        #[clap(subcommand)]
        command: CleanCommands,
    },

    /// Configure Star Citizen root path
    Config {
        #[clap(subcommand)]
        command: ConfigCommands,
    },

    /// Create fresh user config file
    CreateCfg {
        /// LIVE, PTU
        #[clap(long, short)]
        environment: String,

        /// Overwrite cfg, if one exists.
        #[clap(long, short)]
        overwrite: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    /// View saved root path
    View,

    /// Set root path
    Set {
        /// Star Citizen root path
        path: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum CleanCommands {
    /// Delete shader cache
    Shaders,

    /// Delete user folder
    UserFolder(Environment),
}

#[derive(Debug, Parser)]
struct Environment {
    /// LIVE, PTU
    environment: String,
}

fn main() -> Result<(), UtilError> {
    let args = Args::parse();

    match args {
        Args::Clean { command } => clean(command),
        Args::Config { command } => config(command),
        Args::CreateCfg {
            environment,
            overwrite,
        } => create_cfg(&environment, overwrite),
    }
}

fn clean(command: CleanCommands) -> Result<(), UtilError> {
    match command {
        CleanCommands::UserFolder(e) => delete_user_folder(&e.environment)?,
        CleanCommands::Shaders => delete_shaders()?,
    }

    Ok(())
}

fn config(command: ConfigCommands) -> Result<(), UtilError> {
    match command {
        ConfigCommands::Set { path } => set_config(&path)?,
        ConfigCommands::View => view_config()?,
    }

    Ok(())
}
