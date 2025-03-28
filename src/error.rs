use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Error)]
pub enum Error {
    #[error("Invalid length")]
    InvalidLength,
    #[error("Invalid syntax: {0}")]
    Syntax(String),
    #[error("End of file")]
    Eof,
}

#[derive(Debug, Error)]
pub enum RecordParsingError {
    #[error("invalid length")]
    InvalidLength,
    #[error("invalid field {0}: {1}")]
    InvalidField(&'static str, String),
    #[error("non-ascii characters encountered")]
    NonAscii,
    #[error("unexpected record identity, expecting {0}")]
    UnexpectedRecordIdentity(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Syntax(msg.to_string())
    }
}
