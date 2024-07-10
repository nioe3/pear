use std::fmt::{Debug, Formatter};


#[derive(Debug)]
pub enum Error {
    IoError(tokio::io::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
}


impl From<tokio::io::Error> for Error {
    fn from(error: tokio::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::TomlDeError(error)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Error::TomlSerError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO Error: {}", e),
            Error::TomlDeError(e) => write!(f, "Toml deserialization Error: {}", e),
            Error::TomlSerError(e) => write!(f, "Toml serialization Error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            Error::TomlDeError(e) => Some(e),
            Error::TomlSerError(e) => Some(e),
        }
    }
}