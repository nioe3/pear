use std::str::FromStr;
use clap::Subcommand;
use log::error;
use url::Url;
use crate::config;

#[derive(Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ProviderSetupCommands {
    /// configure ollama
    Ollama {
        /// set host address of ollama
        #[arg(short = 'a', long, default_value = "http://127.0.0.1:11434")]
        host: Option<String>,
        /// sets what madel to use
        #[arg(short, long, default_value = "codellama")]
        model: Option<String>
    },
    /// configure anthropic
    Anthropic {
        /// your anthropic api key
        #[arg(short, long, required = true)]
        key: String,

        /// sets what model to use
        #[arg(short, long, required = true)]
        model: String
    }
}

impl ProviderSetupCommands {
    pub async fn execute(mut self) -> anyhow::Result<()> {
        match self {
            ProviderSetupCommands::Ollama { model, host} => {
                let mut config = config::Config::get()
                    .map_err(|e| {
                        error!("Could not read config file: {}", e);
                        e
                    })?;

                if let Some(model) = model {
                    config.provider.ollama.model = model;
                }

                if let Some(host) = host {
                    config.provider.ollama.host = Url::from_str(&host).map_err(|e| {
                        error!("Invalid host Url: {}", e);
                        e
                    })?;
                }

                config.write().await.map_err(|e| {
                    error!("Could not write config file: {}", e);
                    e
                })
            }
            ProviderSetupCommands::Anthropic { key, model } => {
                todo!("Implement anthropic provider");
            }
        }
    }
}