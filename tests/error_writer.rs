use weather::{WeatherReading, Writer};

pub struct ErrorWriter;

#[async_trait::async_trait]
impl Writer for ErrorWriter {
    async fn write(&mut self, _: &WeatherReading) -> Result<(), ()> {
        Err(())
    }
}
