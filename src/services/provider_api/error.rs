use std::fmt::{Debug};

#[derive(thiserror::Error, Debug)]
pub enum ProviderApiError {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Url Parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error)
}