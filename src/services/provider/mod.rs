use clap::builder::PossibleValue;
use clap::ValueEnum;

pub mod anthropic;
pub mod ollama;
mod config;


#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
enum PearProvider {
    Ollama(ollama::Ollama),
}



pub trait Provider {
    fn host_address(&self) -> String;
    fn provider(&self) -> String;
    fn model_name(&self) -> String;
}