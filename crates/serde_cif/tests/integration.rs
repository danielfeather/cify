use serde::{
    Deserialize,
    de::{self, SeqAccess, Visitor},
};

#[derive(Debug)]
struct Header;

#[allow(dead_code)]
struct TestExtract {
    header: Header,
    ti: Vec<TI>,
    ta: Vec<TA>,
    td: Vec<TD>,
}

impl<'de> Deserialize<'de> for TestExtract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TestExtractVisitor;

        impl<'de> Visitor<'de> for TestExtractVisitor {
            type Value = TestExtract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a sequence of one or more records")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let header = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let ti = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let ta = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let td = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(TestExtract { header, ti, ta, td })
            }
        }

        deserializer.deserialize_seq(TestExtractVisitor)
    }
}

#[derive(Debug)]
struct TI;

impl<'de> Deserialize<'de> for TI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let string = <&'de str as serde::Deserialize>::deserialize(deserializer)?;

        let identifier = &string[0..2];

        if identifier != "TI" {
            return Err(serde::de::Error::invalid_type(
                de::Unexpected::StructVariant,
                &"a record type of \"TI\"",
            ));
        }

        Ok(Self)
    }
}

#[derive(Debug)]
struct TA;

impl<'de> Deserialize<'de> for TA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let string = <&'de str as serde::Deserialize>::deserialize(deserializer)?;

        let identifier = &string[0..2];

        if identifier != "TA" {
            return Err(serde::de::Error::invalid_type(
                de::Unexpected::StructVariant,
                &"a record type of \"TI\"",
            ));
        }

        Ok(Self)
    }
}

#[derive(Debug)]
struct TD;

impl<'de> Deserialize<'de> for TD {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let string = <&'de str as serde::Deserialize>::deserialize(deserializer)?;

        let identifier = &string[0..2];

        if identifier != "TD" {
            return Err(serde::de::Error::invalid_type(
                de::Unexpected::StructVariant,
                &"a record type of \"TI\"",
            ));
        }

        Ok(Self)
    }
}

#[derive(Debug)]
struct BS;

impl<'de> Deserialize<'de> for BS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let string = <&'de str as serde::Deserialize>::deserialize(deserializer)?;

        let identifier = &string[0..2];

        if identifier != "BS" {
            return Err(serde::de::Error::invalid_type(
                de::Unexpected::StructVariant,
                &"a record type of \"TI\"",
            ));
        }

        Ok(Self)
    }
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = <&'de str as serde::Deserialize>::deserialize(deserializer)?;

        Ok(Self)
    }
}

#[test]
fn deserialize_multiple_empty_arrays() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read("tests/extract.cif")?;

    let string = String::from_utf8(file)?;

    let extract: TestExtract = serde_cif::from_str(&string)?;

    assert_eq!(extract.ta.len(), 0);

    Ok(())
}

#[test]
fn deserialize_string() -> Result<(), Box<dyn std::error::Error>> {
    let result: Header = serde_cif::from_str(
        "HDTPS.UDFROC1.PD2502282802252154DFROC1B       FA280225280226                    ",
    )?;

    println!("{result:?}");

    Ok(())
}
