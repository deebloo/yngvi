mod station;
mod util;
mod writer;

use hidapi::HidApi;
use influxdb::Client;
use settimeout::set_timeout;
use station::{DeviceIds, Station};
use std::env;
use std::time::Duration;
use writer::Writer;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let influx_addr = env::var("INFLUX_ADDR").unwrap_or(String::from("http://localhost:8086"));
    let client = Client::new(&influx_addr, "weather");
    let writer = Writer::new(&client);

    let mut device_api_ready = false;
    let mut retry_attempts = 0;
    let max_retry_attempts = 5;

    println!("Attempting to create HID API...");

    while !device_api_ready {
        let hid = HidApi::new();

        if let Ok(api) = hid {
            println!("HID API is ready...",);

            device_api_ready = true;

            let device_ids = DeviceIds {
                vid: 0x24c0,
                pid: 0x003,
            };

            let mut station = Station::new(&api, device_ids, &writer);

            println!("Weather Station is ready...");

            station.start().await;
        } else {
            if retry_attempts > max_retry_attempts {
                println!(
                    "There was a problem connecting to the HID API. Retry attempts ({:?}) exceeded",
                    max_retry_attempts
                );
            } else {
                retry_attempts += 1;

                println!(
                    "There was a problem  connecting to the HID API. Retrying. Retry Attempt {:?}",
                    retry_attempts
                );

                set_timeout(Duration::from_secs(10)).await;
            }
        }
    }
}
