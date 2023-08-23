use acurite_influx::InfluxWriter;
use std::env;

#[tokio::main]
async fn main() {
    let source = env::var("AR_SOURCE").unwrap_or("CONSOLE".to_string());

    println!("Application starting with source {}", source);

    let mut writer = InfluxWriter::new();

    match source.to_uppercase().as_str() {
        "CONSOLE" => {
            let mut station = acurite_console::Station::new();

            if let Ok(mut reader) = acurite_core::HidReader::new(0x24c0, 0x003) {
                station.start(&mut reader, &mut writer).await;
            } else {
                println!("Could not start HID Reader");
            }
        }
        "RTL433" => {
            let mut station = acurite_rtl_433::Station::new();

            if let Ok(mut reader) = acurite_rtl_433::RTL433Reader::new() {
                station.start(&mut reader, &mut writer).await;
            } else {
                println!("Could not start RTL433 Reader. Make sure it is installed properly");
            }
        }
        _ => {}
    }
}
