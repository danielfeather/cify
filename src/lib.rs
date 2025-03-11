//! # Common Interface File

mod schedule;
use error::{Error, Result};
use extract::Record;
use serde::de::{self, DeserializeSeed, SeqAccess};

pub mod error;
pub mod extract;

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }

    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = error::Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        map struct enum identifier ignored_any bytes
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

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.lines().count() > 1 {
            return self.deserialize_seq(visitor);
        }

        Err(de::Error::custom(
            "unsupported type provided to deserializer, only str is supported",
        ))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(LineSeparated::new(self))
    }
}

pub fn from_str<'a, T: de::Deserialize<'a>>(input: &'a str) -> error::Result<T> {
    let mut deserializer = Deserializer::from_str(input);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct LineSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> LineSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        LineSeparated { de, first: true }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for LineSeparated<'a, 'de> {
    type Error = error::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek_char().is_err_and(|e| matches!(e, Error::Eof)) {
            return Ok(None);
        }

        // Comma is required before every element except the first.
        if !self.first && self.de.next_char()? != '\n' {
            return Err(Error::Syntax);
        }

        self.first = false;
        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}
