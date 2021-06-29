use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_config<T: DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let mut args = std::env::args();

    let path = &args
        .nth(1)
        .unwrap_or("/etc/acurite-weather/config.json".to_string());

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}
