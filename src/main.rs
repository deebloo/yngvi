// stations
mod console;
mod rtl_433;

mod influx;

use std::io;

struct TestWriter;

#[async_trait::async_trait]
impl acurite_core::Writer for TestWriter {
    async fn write(&mut self, _weather_reading: &acurite_core::WeatherReading) -> Result<(), ()> {
        // println!("{:?}", weather_reading);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Application starting...");

    let mut writer = TestWriter {};
    let mut reader = rtl_433::StdinReader::new();
    let mut station = rtl_433::Station::new();

    println!("Weather Station is ready...");

    station.start(&mut reader, &mut writer).await
}
