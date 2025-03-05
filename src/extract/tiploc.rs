use super::{
    identifier::Crs,
    location::{Nalco, PoMcpCode, Stanox, Tiploc},
};

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

/// TIPLOC Amend Record
pub struct TiplocAmend;

/// TIPLOC Delete Record
pub struct TiplocDelete;
