use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;

pub fn read_config<T: DeserializeOwned + Debug>() -> Result<T, Box<dyn Error>> {
    let mut args = std::env::args();

    let path = &args
        .nth(1)
        .unwrap_or("/etc/acurite/config.json".to_string());

    println!("attempting to read config from {}", path);

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    println!("found config {:?}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn should_return_error_not_panic() {
        #[derive(Deserialize, Debug)]
        pub struct TestConfig;

        assert_eq!(read_config::<TestConfig>().is_err(), true);
    }
}
