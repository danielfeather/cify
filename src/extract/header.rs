use std::{fmt, str::FromStr};

use serde::{
    de::{self, value::StrDeserializer, IntoDeserializer, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use thiserror::Error;

use crate::error::{Error, Result};

#[derive(Error, Debug)]
enum ExtractTypeError {
    #[error("Invalid character")]
    InvalidCharacter,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractType {
    #[serde(rename = "U")]
    Update,
    #[serde(rename = "F")]
    Full,
}

#[derive(Debug)]
/// Struct representing the header record type in a CIF
pub struct Header {
    pub file_mainframe_identity: String,
    pub date_of_extract: String,
    pub time_of_extract: String,
    pub current_file_ref: String,
    pub extract_type: ExtractType,
    pub last_file_ref: String,
    pub version: String,
    pub extract_start_date: String,
    pub extract_end_date: String,
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeaderVisitor;

        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Header")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                let file_mainframe_identity = v[0..20].to_string();
                let date_of_extract = v[20..26].to_string();
                let time_of_extract = v[26..30].to_string();
                let current_file_ref = v[30..37].to_string();
                let last_file_ref = v[37..44].to_string();
                let extract_type_field = &v[44..45];
                let version = v[45..46].to_string();

                let extract_start_date = v[46..52].to_string();
                let extract_end_date = v[52..58].to_string();

                Ok(Header {
                    file_mainframe_identity,
                    date_of_extract,
                    time_of_extract,
                    current_file_ref,
                    last_file_ref,
                    extract_type: ExtractType::deserialize(extract_type_field.into_deserializer())?,
                    version,
                    extract_start_date,
                    extract_end_date,
                })
            }
        }

        deserializer.deserialize_str(HeaderVisitor)
    }
}
