use yngvi::core::{WeatherReading, Writer};

pub struct ErrorWriter;

impl Writer for ErrorWriter {
    async fn write(&mut self, _: &WeatherReading) -> Result<(), ()> {
        Err(())
    }
}
