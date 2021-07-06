mod hid;
mod influx;

use acurite_core::Station;
use hid::HidReader;
use influx::InfluxWriter;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let mut reader = HidReader::new(0x24c0, 0x003);
    let mut writer = InfluxWriter::new();
    let mut station = Station::new();

    println!("Weather Station is ready...");

    station.start(&mut reader, &mut writer).await;
}
