pub mod header;
pub mod identifier;
pub mod location;
pub mod tiploc;

use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "N")]
    New,
    #[serde(rename = "R")]
    Revise,
    #[serde(rename = "D")]
    Delete,
}

/// Type representing the possible record types within a CIF extract
pub enum Record {
    /// Header
    HD(header::Header),
    /// TIPLOC Insert
    TI(tiploc::TiplocInsert),
    /// TIPLOC Amend
    TA(tiploc::TiplocAmend),
    /// TIPLOC Delete
    TD(tiploc::TiplocDelete),
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

pub struct Extract {
    header: header::Header,
}
