mod station;
mod util;
mod writer;
mod writer_influx;

use async_std::task;
use hidapi::HidApi;
use station::{DeviceIds, Station};
use std::time::Duration;
use writer_influx::InfluxWriter;

#[async_std::main]
async fn main() {
    println!("Application starting...");

    let mut device_api_ready = false;
    let mut retry_attempts = 0;
    let max_retry_attempts = 5;

    println!("Attempting to create HID API...");

    while !device_api_ready && retry_attempts < max_retry_attempts {
        match HidApi::new() {
            Ok(api) => {
                println!("HID API is ready...",);

                device_api_ready = true;

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
            Err(_) => {
                retry_attempts += 1;

                println!(
                    "There was a problem  connecting to the HID API. Retrying. Retry Attempt {:?}",
                    retry_attempts
                );

                if retry_attempts == max_retry_attempts {
                    println!("This is the last attempt");
                }

                task::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
