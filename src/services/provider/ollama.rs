use crate::services::provider::Provider;


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Ollama {
    pub host: String,
    pub model_name: String,
    pub provider: String,
}

impl Ollama {
    pub fn new(host: String, model_name: String) -> Self {
        Self {
            host,
            model_name,
            provider: "ollama".to_string(),
        }
    }
}

impl Provider for Ollama {
    fn host_address(&self) -> String {
        self.host.clone()
    }

    fn provider(&self) -> String {
        self.provider.clone()
    }

    fn model_name(&self) -> String {
        self.model_name.clone()
    }


}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            host: "http://127.0.0.1:11434/".to_string(),
            model_name: "codellama".to_string(),
            provider: "ollama".to_string(),
        }
    }
}