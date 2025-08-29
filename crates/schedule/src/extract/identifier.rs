use std::{ops::Deref, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Crs(String);

impl Deref for Crs {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[derive(Debug, Error)]
pub enum CrsParsingError {
    #[error("Crs code must not be longer than 7 characters")]
    InvalidLength,
    #[error("Crs code must not contain any non ascii characters")]
    NonAsciiCharacters,
}

impl FromStr for Crs {
    type Err = CrsParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(CrsParsingError::NonAsciiCharacters);
        }

        if s.len() > 3 {
            return Err(CrsParsingError::InvalidLength);
        }

        Ok(Crs(s.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct Atoc(String);

#[derive(Debug)]
pub struct TrainUid(String);
