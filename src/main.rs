mod station;
mod util;
mod writer;

use hidapi::HidApi;
use influxdb::Client;
use station::{DeviceIds, Station};
use std::env;
use writer::Writer;

#[async_std::main]
async fn main() {
    // connect to the device and create a station

    let hid = HidApi::new();
    let influx_addr = env::var("INFLUX_ADDR").unwrap_or(String::from("http://localhost:8086"));

    let client = Client::new(&influx_addr, "weather");
    let writer = Writer::new(&client);
    let device_ids = DeviceIds {
        vid: 0x24c0,
        pid: 0x003,
    };

    if let Ok(api) = hid {
        let mut station = Station::new(&api, device_ids, &writer);

        station.start().await;
    } else {
        println!("There was a problem connecting to the device");
    }
}
