pub mod header;
pub mod identifier;
pub mod location;
pub mod schedule;
pub mod tiploc;

use header::Header;
use schedule::BasicSchedule;
use serde::{self, Deserialize};
use tiploc::TiplocInsert;

#[derive(Debug, Deserialize, Clone)]
pub enum TransactionType {
    #[serde(rename = "N")]
    New,
    #[serde(rename = "R")]
    Revise,
    #[serde(rename = "D")]
    Delete,
}

/// Type representing the possible record types within a CIF extract
#[derive(Debug, Deserialize, Clone)]
pub enum Record {
    /// Header
    HD(Header),
    /// TIPLOC Insert
    TI(TiplocInsert),
    /// TIPLOC Amend
    TA(String),
    /// TIPLOC Delete
    TD(String),
    /// Association
    AA,
    /// Basic Schedule
    BS(BasicSchedule),
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
