use acurite_core::{WeatherReading, Writer};

pub struct TestWriter {
    pub readings: Vec<WeatherReading>,
}

#[async_trait::async_trait]
impl Writer for TestWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}

pub struct ErrorWriter;

#[async_trait::async_trait]
impl Writer for ErrorWriter {
    async fn write(&mut self, _: &WeatherReading) -> Result<(), ()> {
        Err(())
    }
}
