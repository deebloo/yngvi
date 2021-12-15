// mod hid;
// mod influx;
mod rtl_433;

use std::io;

struct TestWriter;

#[async_trait::async_trait]
impl acurite_core::Writer for TestWriter {
    async fn write(&mut self, weather_reading: &acurite_core::WeatherReading) -> Result<(), ()> {
        println!("{:?}", weather_reading);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Application starting...");

    let mut writer = TestWriter {};
    let mut station = rtl_433::Station::new();

    println!("Weather Station is ready...");

    station.start(&mut writer).await
}
