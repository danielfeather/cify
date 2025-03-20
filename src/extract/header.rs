use std::fmt;

use serde::{
    de::{self, value::StrDeserializer, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::error::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtractType {
    #[serde(rename = "U")]
    Update,
    #[serde(rename = "F")]
    Full,
}

#[derive(Debug)]
/// Struct representing the header record type in a CIF
pub struct Header {
    pub file_mainframe_identity: String,
    pub date_of_extract: String,
    pub time_of_extract: String,
    pub current_file_ref: String,
    pub last_file_ref: String,
    pub extract_type: ExtractType,
    pub version: String,
    pub extract_start_date: String,
    pub extract_end_date: String,
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Secs,
            Nanos,
        }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "secs" => Ok(Field::Secs),
                            "nanos" => Ok(Field::Nanos),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct HeaderVisitor;

        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Header")
            }

            fn visit_seq<V>(self, mut seq: V) -> std::result::Result<Header, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Header {
                    file_mainframe_identity: todo!(),
                    date_of_extract: todo!(),
                    time_of_extract: todo!(),
                    current_file_ref: todo!(),
                    last_file_ref: todo!(),
                    extract_type: todo!(),
                    version: todo!(),
                    extract_start_date: todo!(),
                    extract_end_date: todo!(),
                })
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Header", FIELDS, HeaderVisitor)
    }
}
