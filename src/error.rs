use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Error)]
pub enum Error {
    #[error("Invalid length")]
    InvalidLength,
    #[error("Invalid syntax")]
    Syntax,
    #[error("End of file")]
    Eof,
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        Error::Syntax
    }
}
