use std::cmp::PartialEq;
use std::str::FromStr;
use clap::Parser;
use url::Url;
use crate::cli::app::{Cli};
use crate::cli::app::CliCommands::Provider;
use crate::cli::provider::ProviderCommands;

pub mod services;
pub mod tui;
mod cli;
mod util;

pub mod config;

use services::provider_api::{ProviderApi, ollama};
use crate::cli::provider::setup::ProviderSetupCommands;
use crate::config::MAIN_CONFIG_PATH;


#[tokio::main]
async fn main() {
    // let res = cli::app::App::new().run().await;
    // let c = config::Config::get().unwrap();
    // println!("{:?}", c);

    // let mut ollama = ProviderApi::Ollama(ollama::OllamaRequest::build_from_config().unwrap());
    // let res = ollama.get_completion("why is the sky blue".to_string()).await.unwrap();

    // println!("{}", res);

    let cli = Cli::parse();

    match cli.command {
        Provider(command) => {
            match command {
                ProviderCommands::Setup(commands) => {
                    match commands {
                        ProviderSetupCommands::Ollama { model, host} => {
                            let mut config = config::Config::get().expect("Error getting config");

                            if let Some(model) = model {
                                config.provider.ollama.model = model;
                            }

                            if let Some(host) = host {
                                config.provider.ollama.host = Url::from_str(&host).expect("host me must be a valid url");
                            }

                            config.write().await.expect("Error writing config");
                        }
                        ProviderSetupCommands::Anthropic { key, model } => {
                            todo!("setup command parsing for anthropic for anthropic");
                            println!("Anthropic");
                        }
                    }
                }
            }
        }
    }
}
