use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::services::provider_api::config::ProviderConfig;

mod error;
pub type ConfigError = error::Error;

pub const MAIN_CONFIG_PATH: &str = "/home/nioe/OnlyKino/pear/config/main.toml";


#[derive(Serialize, Deserialize)]
pub struct ConfigPaths {
    pub provider: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub provider: ProviderConfig,
    // pub context: ContextConfig
}

impl Config {
    pub fn get() -> Result<Config, error::Error> {
        let config_path = Path::new(MAIN_CONFIG_PATH);
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub async fn write(&self) -> Result<(), error::Error> {
        println!("{:?}", self);
        let config_path = Path::new(MAIN_CONFIG_PATH);
        let config_str = toml::to_string(&self)?;
        println!("{}", config_str);
        tokio::fs::write(config_path, config_str).await?;
        Ok(())
    }

}


