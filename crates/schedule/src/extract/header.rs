use chrono::NaiveDateTime;
use std::{fmt, str::FromStr};
use thiserror::Error;

use crate::error::RecordParsingError;

#[derive(Error, Debug)]
pub enum ExtractTypeError {
    #[error("Invalid character")]
    InvalidCharacter,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExtractType {
    #[cfg_attr(feature = "serde", serde(rename = "U"))]
    Update,
    #[cfg_attr(feature = "serde", serde(rename = "F"))]
    Full,
}

impl FromStr for ExtractType {
    type Err = ExtractTypeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Update,
            "F" => Self::Full,
            _ => return Err(ExtractTypeError::InvalidCharacter),
        })
    }
}

#[derive(Debug, Clone)]
/// Struct representing the header record type in a CIF
pub struct Header {
    pub file_mainframe_identity: String,
    pub datetime_of_extract: NaiveDateTime,
    pub current_file_ref: String,
    pub extract_type: ExtractType,
    pub last_file_ref: String,
    pub version: String,
    pub extract_start_date: String,
    pub extract_end_date: String,
}

impl FromStr for Header {
    type Err = RecordParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(RecordParsingError::NonAscii);
        }

        let stripped = match s.len() {
            78 => s,
            80 => {
                if &s[0..2] != "HD" {
                    return Err(RecordParsingError::UnexpectedRecordIdentity("HD"));
                }

                &s[2..]
            }
            _ => return Err(RecordParsingError::InvalidLength),
        };

        let file_mainframe_identity = stripped[0..20].to_string();
        let datetime_of_extract_field = &stripped[20..30];
        let current_file_ref = stripped[30..37].to_string();
        let last_file_ref = stripped[37..44].to_string();
        let extract_type_field = &stripped[44..45];
        let version = stripped[45..46].to_string();

        let extract_start_date = stripped[46..52].to_string();
        let extract_end_date = stripped[52..58].to_string();

        let datetime_of_extract =
            NaiveDateTime::parse_from_str(datetime_of_extract_field, "%d%m%y%H%M").unwrap();

        Ok(Header {
            file_mainframe_identity,
            current_file_ref,
            datetime_of_extract,
            last_file_ref,
            extract_type: ExtractType::from_str(extract_type_field).map_err(|_| {
                RecordParsingError::InvalidField("Extract Type", extract_type_field.to_string())
            })?,
            version,
            extract_start_date,
            extract_end_date,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let string = Deserialize::deserialize(deserializer)?;

        Header::from_str(string).map_err(serde::de::Error::custom)
    }
}
