use crate::services::provider_api;
use crate::services::provider_api::ollama::{OllamaRequest};

mod error;
pub mod ollama;
pub mod config;

pub enum ProviderApi {
    Ollama(OllamaRequest)
}

impl ProviderApi {
    pub async fn get_completion(&mut self, context_prompt: String) -> Result<String, error::Error> {
        match self {
            ProviderApi::Ollama(ollama) => {
                ollama.get_completion(&context_prompt).await
            }
        }
    }

    // fn build_from_config(&self) -> Result<ProviderApi, crate::config::ConfigError> {
    //     let config = crate::config::Config::get()?.provider.ollama;
    //
    //     match self {
    //         ProviderApi::Ollama(api) => {
    //             Ok(ProviderApi::Ollama(OllamaRequest {
    //                 host_address: config.host,
    //                 model: config.model,
    //                 ..OllamaRequest::default()
    //             }))
    //         }
    //     }
    // }
}

// pub trait ProviderApi {
//     // todo Implement Context config
//     fn get_completion(&mut self, context_prompt: String) -> Result<String, error::Error>;
//     fn build_from_config() -> Result<Provider, crate::config::ConfigError>;
// }