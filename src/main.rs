mod influx;
mod station;
mod util;
mod writer;

use hidapi::HidApi;
use influx::InfluxWriter;
use station::{DeviceIds, Station};

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let api = HidApi::new().unwrap();
    let writer = InfluxWriter::new();

    let mut station = Station::new(
        &api,
        DeviceIds {
            vid: 0x24c0,
            pid: 0x003,
        },
        &writer,
    );

    println!("Weather Station is ready...");

    station.start().await;
}
