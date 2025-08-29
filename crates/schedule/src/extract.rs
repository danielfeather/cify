pub mod header;
pub mod identifier;
pub mod location;
pub mod schedule;
pub mod tiploc;

pub use header::*;
pub use identifier::*;
pub use location::*;
pub use schedule::*;
pub use tiploc::*;

use serde::{self, Deserialize};

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
    BX(String),
    /// Train Specific Note (Unused)
    TN(String),
    /// Origin Location
    LO(String),
    /// Intermediate Location
    LI(String),
    /// Changes en Route
    CR(String),
    /// Terminating Location
    LT(String),
    /// Location Specific Note (Unused)
    LN(String),
    /// Trailer
    ZZ,
}
