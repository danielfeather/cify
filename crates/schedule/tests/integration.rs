use std::fs;

use cify::{
    extract::{Header, Record},
    timetable::Timetable,
};

#[test]
fn deserialize_array() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Vec<Record>>(&raw)?;

    Ok(())
}

#[test]
fn deserialize_extract() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Timetable>(&raw)?;

    Ok(())
}

#[test]
fn deserialize_string() -> Result<(), Box<dyn std::error::Error>> {
    let result: Header = cify::from_str(
        "TPS.UDFROC1.PD2502282802252154DFROC1B       FA280225280226                    ",
    )?;

    println!("{result:?}");

    Ok(())
}
