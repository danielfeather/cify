use serde::{de::Visitor, Deserialize};

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

impl<'de> Deserialize<'de> for Stanox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StanoxVisitor;

        impl<'de> Visitor<'de> for StanoxVisitor {
            type Value = Stanox;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Stanox")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = v.trim_ascii();

                if v.len() != 5 {
                    return Err(E::custom("expected 5 characters"));
                }

                Ok(Stanox(v.to_string()))
            }
        }

        deserializer.deserialize_str(StanoxVisitor)
    }
}

/// Post Office Location Code (Unused)
#[derive(Debug)]
pub struct PoMcpCode(String);
