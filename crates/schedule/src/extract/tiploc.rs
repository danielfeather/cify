use std::str::FromStr;

use serde::{
    de::{
        self,
        value::{BorrowedStrDeserializer, StrDeserializer},
        IntoDeserializer, Visitor,
    },
    Deserialize,
};

use crate::error::RecordParsingError;

use super::{
    identifier::Crs,
    location::{Nalco, PoMcpCode, Stanox, Tiploc},
};

#[derive(Debug, Clone)]
/// Timing Point Location (TIPLOC) Insert Record
pub struct TiplocInsert {
    pub code: Tiploc,
    pub nalco: Nalco,
    pub tps_description: String,
    pub stanox: Stanox,
    pub po_mcp_code: PoMcpCode,
    pub crs: Crs,
    pub description: String,
}

impl FromStr for TiplocInsert {
    type Err = RecordParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(RecordParsingError::NonAscii);
        }

        let stripped = match s.len() {
            78 => s,
            80 => {
                if &s[0..2] != "TI" {
                    return Err(RecordParsingError::UnexpectedRecordIdentity("BS"));
                }

                &s[2..]
            }
            _ => return Err(RecordParsingError::InvalidLength),
        };

        Ok(TiplocInsert {
            code: stripped[0..7].parse().map_err(|_| {
                RecordParsingError::InvalidField("TIPLOC", stripped[0..7].to_string())
            })?,
            nalco: stripped[9..15].parse().map_err(|_| {
                RecordParsingError::InvalidField(
                    "National Location Code",
                    stripped[9..15].to_string(),
                )
            })?,
            tps_description: stripped[16..42].to_string(),
            stanox: Stanox::from_str(&stripped[42..47]).map_err(|_| {
                RecordParsingError::InvalidField("Stanox", stripped[42..47].to_string())
            })?,
            po_mcp_code: PoMcpCode::from_str(&stripped[47..51]).map_err(|_| {
                RecordParsingError::InvalidField("PO MCP Code", stripped[46..50].to_string())
            })?,
            crs: Crs::from_str(&stripped[51..54]).map_err(|_| {
                RecordParsingError::InvalidField("CRS Code", stripped[50..53].to_string())
            })?,
            description: stripped[54..70].to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for TiplocInsert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_str(Deserialize::deserialize(deserializer)?).map_err(|e| de::Error::custom(e))
    }
}

#[test]
fn deserialize_ti() -> Result<(), Box<dyn std::error::Error>> {
    let raw = "TIAACHEN 00081601LAACHEN                    00005   0                           ";
    let deserializer = BorrowedStrDeserializer::<de::value::Error>::new(raw);

    let _ = TiplocInsert::deserialize(deserializer)?;

    Ok(())
}

#[derive(Debug)]
/// TIPLOC Amend Record
pub struct TiplocAmend;

#[derive(Debug)]
/// TIPLOC Delete Record
pub struct TiplocDelete;
