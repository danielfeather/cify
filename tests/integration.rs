use std::fs;

use std::io::Error;

use cify::extract::Record;

#[test]
fn deserialize_array() -> Result<(), Error> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Vec<Record>>(&raw).unwrap();

    println!("Result: {:?}", result);

    Ok(())
}
