// stations
mod console;
mod rtl_433;

mod influx;

use acurite_core::config;
use serde::Deserialize;

struct TestWriter;

#[async_trait::async_trait]
impl acurite_core::Writer for TestWriter {
    async fn write(&mut self, _weather_reading: &acurite_core::WeatherReading) -> Result<(), ()> {
        // println!("{:?}", weather_reading);

        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Station {
    CONSOLE,
    RTL433,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AcuriteConfig {
    pub station: Station,
}

#[tokio::main]
async fn main() {
    println!("Application starting...");

    // Read configuration. Default to read from the console
    let program_config = config::read_config::<AcuriteConfig>().unwrap_or(AcuriteConfig {
        station: Station::RTL433,
    });

    match program_config.station {
        Station::CONSOLE => {
            let mut writer = influx::InfluxWriter::new();
            let mut reader = console::HidReader::new(0x24c0, 0x003);
            let mut station = console::Station::new();

            println!("Weather Station is ready...");

            station.start(&mut reader, &mut writer).await;
        }
        Station::RTL433 => {
            let mut writer = TestWriter {};
            let mut reader = rtl_433::StdinReader::new();
            let mut station = rtl_433::Station::new();

            println!("Weather Station is ready...");

            station.start(&mut reader, &mut writer).await;
        }
    }
}
