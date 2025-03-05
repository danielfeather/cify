use serde::{de, Deserialize, Serialize};

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

pub struct HeaderVisitor;

impl<'de> de::Visitor<'de> for HeaderVisitor {
    type Value = Header;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a command in the format '+:<cmd> <required_key> <optional_value>'")
    }

    fn visit_str<E>(self, record_str: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        if record_str.len() != 78 {
            return Err(de::Error::custom(
                "invalid header record provided, length must be 78 characters",
            ));
        }

        if !record_str.is_ascii() {
            return Err(de::Error::custom(
                "invalid header record provided, string contains non-ascii characters",
            ));
        }

        let file_mainframe_identity = record_str[0..20].to_owned();

        let date_of_extract = record_str[20..26].to_owned();
        let time_of_extract = record_str[26..30].to_owned();

        let current_file_ref = record_str[30..37].to_owned();
        let last_file_ref = record_str[37..43].to_owned();

        let version = record_str[45..46].to_owned();
        let extract_start_date = record_str[46..52].to_owned();
        let extract_end_date = record_str[52..58].to_owned();

        Ok(Header {
            file_mainframe_identity,
            current_file_ref,
            last_file_ref,
            date_of_extract,
            time_of_extract,
            extract_type: ExtractType::Full,
            version,
            extract_end_date,
            extract_start_date,
        })
    }
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        seq map struct enum identifier ignored_any bytes
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.input = self
            .input
            .strip_suffix("\r\n")
            .or(self.input.strip_suffix("\n"))
            .unwrap_or(self.input);

        visitor.visit_str::<Self::Error>(&self.input)
    }

    fn deserialize_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::custom(
            "unsupported type provided to deserializer, only str is supported",
        ))
    }
}

pub fn deserialize<'a, T: de::Deserialize<'a>>(input: &'a str) -> Result<T> {
    let mut deserializer = Deserializer::from_str(input);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

impl<'de> de::Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(HeaderVisitor)
    }
}

#[test]
fn test_deserialize_set() {
    let data = r"TPS.UDFROC1.PD2502282802252154DFROC1B       FA280225280226                    ";

    let result: Header = deserialize(data).expect("Invalid header");

    println!("{:#?}", result)
}
