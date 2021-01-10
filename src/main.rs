mod station;
mod util;
mod writer;

use hidapi::HidApi;
use influxdb::Client;
use settimeout::set_timeout;
use station::Station;
use std::env;
use std::time::Duration;
use writer::Writer;

#[async_std::main]
async fn main() {
    // connect to the device and create a station
    let (vid, pid) = (0x24c0, 0x003);
    let hid = HidApi::new().unwrap();
    let influx_addr = env::var("INFLUX_ADDR").unwrap_or(String::from("http://localhost:8086"));

    let mut connected = false;
    let mut retry_count = 0;
    let mut retry_delay = 5; // in seconds

    while !connected {
        let device = hid.open(vid, pid);

        println!("Attempting to connect to device...");

        if let Ok(value) = device {
            println!("Device connected!");

            connected = true;

            let client = Client::new(&influx_addr, "weather");
            let writer = Writer::new(&client);

            let mut station = Station::new(&value, &writer);

            station.start().await;
        } else {
            if retry_count < 5 {
                // If failed to connect wait a few seconds and try again
                set_timeout(Duration::from_secs(retry_delay)).await;

                retry_count += 1; // keep track of how many times we have retried
                retry_delay += 5; // wait a bit longer each attempt to give the program a better chance
            } else {
                panic!("Failed to connect to device.")
            }
        }
    }
}
