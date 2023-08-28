use display::{DisplayReader, HidSource};
use influx_db::InfluxWriter;
use rtl_433::{rtl_433_source, RTL433Reader};
use std::env;
use weather::Station;

#[tokio::main]
async fn main() {
    let source = env::var("WEATHER_SOURCE").unwrap_or("CONSOLE".to_string());

    let mut station = Station::new();
    let mut writer = InfluxWriter::new();

    match source.to_uppercase().as_str() {
        "CONSOLE" => {
            let hid = HidSource::new(0x24c0, 0x003).expect("could not start HID Api");
            let reader = DisplayReader::new(hid);

            station.start(reader, &mut writer).await;
        }
        "RTL433" => {
            let reader = RTL433Reader::new(rtl_433_source());

            station.start(reader, &mut writer).await;
        }
        _ => {}
    }
}
