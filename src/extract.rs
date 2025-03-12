pub mod header;
pub mod identifier;
pub mod location;
pub mod tiploc;

use serde::{self, de, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "N")]
    New,
    #[serde(rename = "R")]
    Revise,
    #[serde(rename = "D")]
    Delete,
}

#[derive(Debug)]
#[serde()]
/// Type representing the possible record types within a CIF extract
pub enum Record {
    /// Header
    HD(String),
    /// TIPLOC Insert
    TI(String),
    /// TIPLOC Amend
    TA(String),
    /// TIPLOC Delete
    TD(String),
    /// Association
    AA,
    /// Basic Schedule
    BS,
    /// Basic Schedule Extra Details
    BX,
    /// Train Specific Note (Unused)
    TN,
    /// Origin Location
    LO,
    /// Intermediate Location
    LI,
    /// Changes en Route
    CR,
    /// Terminating Location
    LT,
    /// Location Specific Note (Unused)
    LN,
    /// Trailer
    ZZ,
}

pub struct RecordVisitor;

impl<'de> de::Visitor<'de> for RecordVisitor {
    type Value = Record;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a record starting with a valid record identity'")
    }

    fn visit_str<E>(self, record_str: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        println!("Record Str: {:#?}", record_str);
        Ok(Record::ZZ)
    }
}

impl<'de> de::Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(RecordVisitor)
    }
}
