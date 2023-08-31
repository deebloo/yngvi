use std::env;
use ws_core::{InMemWriter, Station, StdoutWriter, WeatherReading};
use ws_display::{DisplayReader, HidSource};
use ws_influx_db::InfluxWriter;
use ws_rtl_433::{rtl_433_source, RTL433Reader};

enum AppWriter {
    Influx(InfluxWriter),
    Stdout(StdoutWriter),
    InMemory(InMemWriter),
}

#[tokio::main]
async fn main() {
    let source = env::var("WS_SOURCE").unwrap_or("CONSOLE".to_string());
    let dest = env::var("WS_DEST").unwrap_or("STDOUT".to_string());

    let mut station = Station::new();
    let mut reader = find_reader(source.to_uppercase().as_str());
    let mut writer = find_writer(dest.to_uppercase().as_str());

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    match &mut writer {
        AppWriter::Influx(writer) => station.start(&mut reader, writer).await,
        AppWriter::InMemory(writer) => station.start(&mut reader, writer).await,
        AppWriter::Stdout(writer) => station.start(&mut reader, writer).await,
    }
}

fn find_reader(value: &str) -> Box<dyn Iterator<Item = WeatherReading>> {
    match value {
        "CONSOLE" => Box::new(DisplayReader::new(
            HidSource::new(0x24c0, 0x003).expect("could not start HID Api"),
        )),
        "RTL433" => Box::new(RTL433Reader::new(rtl_433_source())),
        _ => panic!("no reader defined. found {}", value),
    }
}

fn find_writer(value: &str) -> AppWriter {
    match value {
        "INFLUXDB" => AppWriter::Influx(InfluxWriter::new()),
        "INMEMORY" => AppWriter::InMemory(InMemWriter::new()),
        "STDOUT" => AppWriter::Stdout(StdoutWriter::new()),
        _ => panic!("no writer defined. found {}", value),
    }
}
