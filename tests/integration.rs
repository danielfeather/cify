use std::fs;

use cify::extract::{header::Header, Record};

#[test]
fn deserialize_array() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Header>(&raw)?;

    println!(
        "{:?}",
        result // .iter()
               // .filter(|&record| matches!(record, Record::BS(_)))
               // .collect::<Vec<&Record>>()
    );

    Ok(())
}
