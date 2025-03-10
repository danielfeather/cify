/// National Location Code
#[derive(Debug)]
pub struct Nalco(String);

/// Representation of a TIPLOC (Timing Point Location Code)
#[derive(Debug)]
pub struct Tiploc(String);

/// Stanox
///
/// TOPS location code
#[derive(Debug)]
pub struct Stanox(String);

impl Stanox {
    pub fn is_empty(&self) -> bool {
        self.0 == "00000"
    }
}

/// Post Office Location Code (Unused)
#[derive(Debug)]
pub struct PoMcpCode(String);
