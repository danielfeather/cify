/// National Location Code
pub struct Nalco(String);

/// Representation of a TIPLOC (Timing Point Location Code)
pub struct Tiploc(String);

/// Stanox
///
/// TOPS location code
pub struct Stanox(String);

impl Stanox {
    pub fn is_empty(&self) -> bool {
        self.0 == "00000"
    }
}

/// Post Office Location Code (Unused)
pub struct PoMcpCode(String);
