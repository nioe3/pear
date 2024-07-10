use serde::{Deserialize, Serialize};
use super::ollama;


#[derive(Serialize, Deserialize, Debug)]
pub struct ProviderConfig {
    pub(crate) ollama: ollama::config::OllamaConfig,
}
