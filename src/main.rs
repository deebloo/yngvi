use acurite_core::config;
use acurite_influx::InfluxWriter;
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
        station: Station::CONSOLE,
    });

    println!("Booking up with config: {:?}", program_config);

    let mut writer = InfluxWriter::new();

    match program_config.station {
        Station::CONSOLE => {
            let mut reader = acurite_core::HidReader::new(0x24c0, 0x003);
            let mut station = acurite_console::Station::new();

            station.start(&mut reader, &mut writer).await;
        }
        Station::RTL433 => {
            let mut reader = acurite_rtl_433::RTL433Reader::new().unwrap();
            let mut station = acurite_rtl_433::Station::new();

            station.start(&mut reader, &mut writer).await;
        }
    }
}
