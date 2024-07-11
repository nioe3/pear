pub mod config;
use serde::{Deserialize, Serialize};
use serde_json;
use super::error::ProviderApiError;

#[derive(Serialize, Deserialize)]
pub struct RawOllamaRequest {
    pub prompt: String,
    pub model: String,
    pub stream: bool
}
#[derive(Serialize, Deserialize)]
pub struct OllamaRequest {
    pub(crate) host_address: url::Url,

    pub prompt: String,
    pub model: String,
    // pub format: Option<OllamaApiFormat>,
    pub system: Option<String>, // overwrites the model files message used to specify custom behavior
    // pub template: Option<String>
    pub context: bool, // takes context from prom previous request for /generate
    pub stream: bool,
    pub raw: bool, // if true no format for the response
    pub keep_alive: Option<u64>, // controls how long the model will stay in the memory following request; default 5m
}

impl OllamaRequest {

    pub fn new(prompt: String) -> OllamaRequest {
        Self {
            prompt,
            ..Default::default()
        }
    }

    pub async fn get_completion(&self, context_prompt: &str) -> Result<String, ProviderApiError> {

        let request = RawOllamaRequest {
            prompt: context_prompt.to_string(),
            model: self.model.clone(),
            stream: self.stream
        };

        let req_json = serde_json::to_string(&request)?;

        let client = reqwest::Client::new();
        let endpoint = self.host_address.join("api/generate")?;


        let res = client.post(endpoint)
            .body(req_json)
            .send()
            .await?
            .text()
            .await?;



        Ok(res)
    }

    pub fn build_from_config() -> anyhow::Result<OllamaRequest> {
        let config = crate::config::Config::get()?.provider.ollama;

        Ok(OllamaRequest {
            host_address: config.host,
            model: config.model,
            ..OllamaRequest::default()
        })
    }
}


impl Default for OllamaRequest {
    fn default() -> Self {
        Self {
            host_address: url::Url::parse("http://127.0.0.1:11434").unwrap(),
            prompt: "".to_string(),
            model: "codellama".to_string(),
            // format: None,
            system: None,
            context: false,
            stream: false,
            raw: false,
            keep_alive: Some(5)
        }
    }
}



