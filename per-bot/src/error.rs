use std::convert::From;
use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BotError>;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Error while reading configuration")]
    Io(#[source] io::Error),
    #[error("Error while parsing the configuration")]
    Toml(#[source] toml::de::Error),
}

impl From<io::Error> for BotError {
    fn from(e: io::Error) -> Self {
        BotError::Io(e)
    }
}

impl From<toml::de::Error> for BotError {
    fn from(e: toml::de::Error) -> Self {
        BotError::Toml(e)
    }
}
