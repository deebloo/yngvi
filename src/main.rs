// stations
mod influx;

use acurite_core::config;
use serde::Deserialize;

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
            let mut reader = acurite_console::HidReader::new(0x24c0, 0x003);
            let mut station = acurite_console::Station::new();

            println!("Weather Station is ready...");

            station.start(&mut reader, &mut writer).await;
        }
        Station::RTL433 => {
            let mut writer = influx::InfluxWriter::new();
            let mut reader = acurite_rtl_433::StdinReader::new();
            let mut station = acurite_rtl_433::Station::new();

            println!("Weather Station is ready...");

            station.start(&mut reader, &mut writer).await;
        }
    }
}
