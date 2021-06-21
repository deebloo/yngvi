mod hid;
mod influx;
mod reader;
mod station;
mod formulas;
mod writer;

use hid::HidReader;
use influx::InfluxWriter;
use station::Station;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let reader = HidReader::new(0x24c0, 0x003);
    let writer = InfluxWriter::new();
    let mut station = Station::new();

    println!("Weather Station is ready...");

    station.start(&reader, &writer).await;
}
