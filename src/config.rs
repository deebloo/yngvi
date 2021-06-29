use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_config<T: DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let path = &args[1];

    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let config = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(config)
}
