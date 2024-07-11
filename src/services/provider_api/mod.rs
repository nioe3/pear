pub mod anthropic;
pub mod error;
pub mod ollama;
pub mod config;

use ollama::OllamaRequest;
use error::ProviderApiError;

pub enum ProviderApi {
    Ollama(OllamaRequest),
    // Anthropic(AnthropicRequest)
}

impl ProviderApi {
    pub async fn get_completion(&mut self, context_prompt: String) -> Result<String, ProviderApiError>{
        match self {
            ProviderApi::Ollama(ollama) => {
                ollama.get_completion(&context_prompt).await
            },
            // ProviderApi::Anthropic(anthropic) => {
            //     anthropic.get_completion(&context_prompt).await
            // }
        }
    }

}

