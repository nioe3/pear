use std::fmt;
use std::fmt::{Debug, Formatter};


#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    UrlParseError(url::ParseError),
    SerdeError(serde_json::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            Error::UrlParseError(e) => write!(f, "Url Parse Error: {}", e),
            Error::SerdeError(e) => write!(f, "Serde Error: {}", e)
        }
    }
}


impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReqwestError(e) => Some(e),
            Error::UrlParseError(e) => Some(e),
            Error::SerdeError(e) => Some(e)
        }
    }

}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::UrlParseError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeError(error)
    }
}