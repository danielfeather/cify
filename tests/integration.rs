use std::fs;

use cify::extract::Record;

#[test]
fn deserialize_array() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Vec<Record>>(&raw)?;

    println!(
        "{:#?}",
        result // .iter()
               // .filter(|&record| matches!(record, Record::BS(_)))
               // .collect::<Vec<&Record>>()
    );

    Ok(())
}
