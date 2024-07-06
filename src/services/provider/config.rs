use crate::services::provider::PearProvider;

const CONFIG_PATH: &str = "config/provider.json";


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    models: Vec<PearProvider>,
}
