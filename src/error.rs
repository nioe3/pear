

#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Config Error: {0}")]
    ConfigError(#[from] ConfigError),


    #[error("Provider Error: {0}")]
    ProviderError(#[from] ProviderError),

}