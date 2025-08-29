use std::{ops::Deref, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use thiserror::Error;

/// National Location Code
#[derive(Debug, Clone)]
pub struct Nalco(String);

#[derive(Debug, Error)]
pub enum NalcoParsingError {
    #[error("Nalco must not be longer than 6 characters")]
    InvalidLength,
    #[error("Nalco must not contain any non ascii characters")]
    NonAsciiCharacters,
}

impl FromStr for Nalco {
    type Err = NalcoParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(NalcoParsingError::NonAsciiCharacters);
        }

        if s.len() > 6 {
            return Err(NalcoParsingError::InvalidLength);
        }

        let mut string = String::with_capacity(s.len());

        string.push_str(s);

        Ok(Self(string))
    }
}

impl Deref for Nalco {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

/// Representation of a TIPLOC (Timing Point Location Code)
#[derive(Debug, Clone)]
pub struct Tiploc(String);

impl Deref for Tiploc {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[derive(Debug, Error)]
pub enum TiplocParsingError {
    #[error("Tiploc must not be longer than 7 characters")]
    InvalidLength,
    #[error("Tiploc must not contain any non ascii characters")]
    NonAsciiCharacters,
}

impl FromStr for Tiploc {
    type Err = TiplocParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 7 {
            return Err(TiplocParsingError::InvalidLength);
        }

        let mut string = String::with_capacity(7);

        string.push_str(s);

        Ok(Self(string))
    }
}

/// Stanox
///
/// TOPS location code
#[derive(Debug, Clone)]
pub struct Stanox(String);

impl Deref for Stanox {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[derive(Debug, Error)]
pub enum StanoxParsingError {
    #[error("Stanox must not be longer than 7 characters")]
    InvalidLength,
    #[error("Stanox must not contain any non ascii characters")]
    NonAsciiCharacters,
}

impl FromStr for Stanox {
    type Err = StanoxParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 5 {
            return Err(StanoxParsingError::InvalidLength);
        }

        let mut string = String::with_capacity(s.len());

        string.push_str(s);

        Ok(Self(string))
    }
}

impl Stanox {
    pub fn is_empty(&self) -> bool {
        self.0 == "00000"
    }
}

impl<'de> Deserialize<'de> for Stanox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct StanoxVisitor;

        impl<'de> Visitor<'de> for StanoxVisitor {
            type Value = Stanox;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Stanox")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<Self::Value>().map_err(|e| de::Error::custom(e))
            }
        }

        deserializer.deserialize_str(StanoxVisitor)
    }
}

#[derive(Debug, Error)]
pub enum PoMcpCodeParsingError {
    #[error("PO MCP Code must not be longer than 7 characters")]
    InvalidLength,
    #[error("PO MCP Code must not contain any non ascii characters")]
    NonAsciiCharacters,
}

/// Post Office Location Code (Unused)
#[derive(Debug, Clone)]
pub struct PoMcpCode(String);

impl PoMcpCode {
    fn is_empty(&self) -> bool {
        self.0 == "0000" || self.0 == "   0"
    }
}

impl FromStr for PoMcpCode {
    type Err = PoMcpCodeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(PoMcpCodeParsingError::NonAsciiCharacters);
        }

        if s.len() > 4 {
            return Err(PoMcpCodeParsingError::InvalidLength);
        }

        Ok(PoMcpCode(s.to_string()))
    }
}

#[derive(Debug)]
pub struct OriginLocation {}

#[derive(Debug)]
pub struct IntermediateLocation {}

#[derive(Debug)]
pub struct TerminatingLocation {}
