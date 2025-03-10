use serde::Deserialize;

use super::{
    identifier::Crs,
    location::{Nalco, PoMcpCode, Stanox, Tiploc},
};

#[derive(Debug)]
/// TIPLOC Insert Record
pub struct TiplocInsert {
    code: Tiploc,
    nalco: Nalco,
    tps_description: String,
    stanox: Stanox,
    po_mcp_code: PoMcpCode,
    crs: Crs,
    capri_description: String,
}

#[derive(Debug)]
/// TIPLOC Amend Record
pub struct TiplocAmend;

#[derive(Debug)]
/// TIPLOC Delete Record
pub struct TiplocDelete;
