use std::{ops::Deref, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize,
};
use thiserror::Error;

use crate::error::RecordParsingError;

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
pub struct OriginLocation {
    // location: Tiploc,
    scheduled_departure_time: String,
}

impl FromStr for OriginLocation {
    type Err = RecordParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(RecordParsingError::NonAscii);
        }

        let stripped = match s.len() {
            78 => s,
            80 => {
                if &s[0..2] != "LO" {
                    return Err(RecordParsingError::UnexpectedRecordIdentity("LO"));
                }

                &s[2..]
            }
            _ => return Err(RecordParsingError::InvalidLength),
        };

        Ok(Self {
            // location: Tiploc::from_str(&stripped[0..8])
            //     .map_err(|e| RecordParsingError::InvalidField("location", e.to_string()))?,
            scheduled_departure_time: stripped[8..13].to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for OriginLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Self::from_str(Deserialize::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Debug)]
pub struct IntermediateLocation;

impl FromStr for IntermediateLocation {
    type Err = RecordParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = match s.len() {
            78 => s,
            80 => {
                if &s[0..2] != "LI" {
                    return Err(RecordParsingError::UnexpectedRecordIdentity("LI"));
                }

                &s[2..]
            }
            _ => return Err(RecordParsingError::InvalidLength),
        };

        Ok(IntermediateLocation)
    }
}

impl<'de> Deserialize<'de> for IntermediateLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Self::from_str(Deserialize::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[derive(Debug)]
pub struct TerminatingLocation;

impl<'de> Deserialize<'de> for TerminatingLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let _string: &str = Deserialize::deserialize(deserializer)?;
        Ok(Self)
    }
}
