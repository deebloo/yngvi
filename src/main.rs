mod station;
mod util;
mod writer;

use hidapi::HidApi;
use influxdb::Client;
use station::Station;
use std::env;
use writer::Writer;

#[async_std::main]
async fn main() {
    // connect to the device and create a station
    let (vid, pid) = (0x24c0, 0x003);
    let hid = HidApi::new().unwrap();
    let device = hid.open(vid, pid).unwrap();

    let influx_addr = env::var("INFLUX_ADDR").unwrap_or(String::from("http://localhost:8086"));
    let client = Client::new(influx_addr, "weather");
    let writer = Writer::new(&client);
    let station = Station::new(&device);

    station.start(&writer).await;
}
