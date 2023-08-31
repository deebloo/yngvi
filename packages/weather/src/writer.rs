use async_trait::async_trait;

use crate::WeatherReading;

#[async_trait]
pub trait Writer {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()>;
}

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Writer for StdoutWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        if let Ok(json) = serde_json::to_string(weather_reading) {
            println!("{}", json);
        }

        Ok(())
    }
}

pub struct InMemWriter {
    pub readings: Vec<WeatherReading>,
}

impl InMemWriter {
    pub fn new() -> Self {
        Self { readings: vec![] }
    }
}

#[async_trait]
impl Writer for InMemWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}
