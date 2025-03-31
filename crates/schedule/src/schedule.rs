use serde::Deserialize;

use crate::extract::header::Header;

#[derive(Debug)]
/// Type representing a schedule
///
/// To be a valid schedule, the input must contain a header and a trailer
/// record type
pub struct Schedule {
    pub header: Header,
}

impl Schedule {
    /// Returns `true` if the schedule only contains a header and a trailer record type
    pub fn is_empty() {}
}

// impl<'de> Deserialize<'de> for Schedule {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let result = deserializer.deserialize_seq(visitor)
//     }
// }
