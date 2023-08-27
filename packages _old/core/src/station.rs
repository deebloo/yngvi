use crate::{RetryManager, WeatherReading};

pub struct Station {
    pub weather_reading: WeatherReading,
    pub retry_manager: RetryManager,
}

impl Station {
    pub fn new() -> Self {
        Self {
            weather_reading: WeatherReading::new(),
            retry_manager: RetryManager::new(),
        }
    }

    pub async fn start<T: Iterator<Item = String>>(&mut self, reader: T) {
        for item in reader {
            println!("{}", item);
        }
    }
}
