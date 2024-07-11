use std::fmt::{Debug};

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {

    #[error("Tokio IO Error: {0}")]
    IoError(#[from] tokio::io::Error),

    #[error("Toml deserialization Error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("Toml serialization Error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
}