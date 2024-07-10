use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaConfig {
    pub host: url::Url,
    pub model: String
}
