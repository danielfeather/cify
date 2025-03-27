pub mod header;
pub mod identifier;
pub mod location;
pub mod tiploc;

use header::Header;
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "N")]
    New,
    #[serde(rename = "R")]
    Revise,
    #[serde(rename = "D")]
    Delete,
}

/// Type representing the possible record types within a CIF extract
#[derive(Debug, Deserialize)]
pub enum Record {
    /// Header
    HD(Header),
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
