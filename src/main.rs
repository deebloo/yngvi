mod config;
mod formulas;
mod hid;
mod influx;
mod reader;
mod station;
mod writer;

use hid::HidReader;
use influx::InfluxWriter;
use station::Station;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let mut reader = HidReader::new(0x24c0, 0x003);
    let mut writer = InfluxWriter::new();
    let mut station = Station::new();

    println!("Weather Station is ready...");

    station.start(&mut reader, &mut writer).await;
}
