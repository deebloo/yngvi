use async_trait::async_trait;

use crate::WeatherReading;

#[async_trait]
pub trait Writer {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()>;
}

pub struct StdoutWriter;

#[async_trait]
impl Writer for StdoutWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        println!("{}", weather_reading);

        Ok(())
    }
}

pub struct InMemWriter {
    pub readings: Vec<WeatherReading>,
}

#[async_trait]
impl Writer for InMemWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}
