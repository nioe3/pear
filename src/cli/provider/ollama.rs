use clap::{Arg, ArgGroup, ArgMatches, Command};
use crate::cli::provider::{SetupCommandManager};
use crate::services::provider::ollama::Ollama;
use crate::services::provider::Provider;


pub struct OllamaCommandManager;

impl SetupCommandManager for OllamaCommandManager {
    fn read_provider_name(&self) -> &'static str {
        "ollama"
    }
    fn setup_args(&self) -> Vec<Arg> {
        let ollama = Ollama::default();
        vec![
            Arg::new("model-name")
                .long("model")
                .short('m')
                .value_name("MODEL NAME")
                .help("Sets the model for the provider to use")
                .default_value(ollama.model_name)
                .required(false),

            Arg::new("host-address")
                .long("host")
                .short('a')
                .value_name("HOST ADDRESS")
                .help("Sets the host address of the ollama API")
                .default_value(ollama.host)
                .required(false)
        ]

    }

    fn setup_matches(&self, matches: &ArgMatches) -> std::io::Result<()> {
        if let Some(name) = matches.get_one::<String>("model-name") {
            todo!("Add model")
        }
        if let Some(host) = matches.get_one::<String>("host-address") {
            todo!("Add host address")
        }
        Ok(())
    }
}


