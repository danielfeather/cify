use std::fs;

use std::io::Error;

use cify::extract::Record;

#[test]
fn deserialize_array() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("tests/extract.cif")?;

    let result = cify::from_str::<Record>(&raw)?;

    let Record::HD(data) = result else {
        return Err(Box::new(cify::error::Error::Eof));
    };

    println!("{}", data.len());

    Ok(())
}
