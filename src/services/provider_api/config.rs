use serde::{Deserialize, Serialize};
use super::ollama;
use super::anthropic;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderConfig {
    pub(crate) ollama: ollama::config::OllamaConfig,
    pub(crate) anthropic: anthropic::config::AnthropicConfig
}