use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_config<T: DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let path = &args[1];

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}
