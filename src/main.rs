mod station;
mod util;
mod writer;

use async_std::task;
use hidapi::HidApi;
use station::{DeviceIds, Station};
use std::time::Duration;
use writer::InfluxWriter;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let mut device_api_ready = false;
    let mut retry_attempts = 0;
    let max_retry_attempts = 5;

    println!("Attempting to create HID API...");

    while !device_api_ready {
        let hid = HidApi::new();

        if let Ok(api) = hid {
            println!("HID API is ready...",);

            device_api_ready = true;

            let writer = InfluxWriter::new();
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

                task::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
