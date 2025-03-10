use std::os::macos::raw;

use serde::{
    de::{self, value::StrDeserializer},
    Deserialize, Serialize,
};

use crate::error::{Error, Result};

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
    pub last_file_ref: String,
    pub extract_type: ExtractType,
    pub version: String,
    pub extract_start_date: String,
    pub extract_end_date: String,
}

pub struct HeaderVisitor;

impl<'de> de::Visitor<'de> for HeaderVisitor {
    type Value = Header;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a command in the format '+:<cmd> <required_key> <optional_value>'")
    }

    fn visit_str<E>(self, record_str: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if record_str.len() != 78 {
            return Err(de::Error::custom(
                "invalid header record provided, length must be 78 characters",
            ));
        }

        if !record_str.is_ascii() {
            return Err(de::Error::custom(
                "invalid header record provided, string contains non-ascii characters",
            ));
        }

        let file_mainframe_identity = record_str[0..20].to_owned();

        let date_of_extract = record_str[20..26].to_owned();
        let time_of_extract = record_str[26..30].to_owned();

        let current_file_ref = record_str[30..37].to_owned();
        let last_file_ref = record_str[37..43].to_owned();

        let raw_extract_type: &str = &record_str[43..44];
        let version = record_str[45..46].to_owned();
        let extract_start_date = record_str[46..52].to_owned();
        let extract_end_date = record_str[52..58].to_owned();

        // let extract_type = self.visit_enum(raw_extract_type);

        Ok(Header {
            file_mainframe_identity,
            current_file_ref,
            last_file_ref,
            date_of_extract,
            time_of_extract,
            extract_type: ExtractType::Full,
            version,
            extract_end_date,
            extract_start_date,
        })
    }
}

impl<'de> de::Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(HeaderVisitor)
    }
}
