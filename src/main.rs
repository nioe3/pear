use std::str::FromStr;
use clap::Parser;
use log::{error, info, warn};
use url::Url;
use crate::cli::app::{Cli};
use crate::cli::app::CliCommands::Provider;
use crate::cli::provider::ProviderCommands;

pub mod services;
pub mod tui;
mod cli;
mod util;

pub mod config;

use crate::cli::provider::setup::ProviderSetupCommands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::Builder::from_env(env_logger::Env::default())
        // .target(env_logger::Target::Stderr)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Provider(command) => {
            match command {
                ProviderCommands::Setup(commands) => {

                }
            }
        }
    }

    Ok(())
}
