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
    let source: String = env::var("WEATHER_SOURCE").unwrap_or("CONSOLE".to_string());
    let dest: String = env::var("WEATHER_DEST").unwrap_or("INFLUXDB".to_string());

    let mut station = Station::new();
    let reader = find_reader(source.to_uppercase().as_str());
    let mut writer = find_writer(dest.to_uppercase().as_str());

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    match &mut writer {
        AppWriter::Influx(writer) => station.start(reader, writer).await,
        AppWriter::InMemory(writer) => station.start(reader, writer).await,
        AppWriter::Stdout(writer) => station.start(reader, writer).await,
    }
}

fn find_reader(value: &str) -> Box<dyn Iterator<Item = WeatherReading>> {
    match value {
        "CONSOLE" => Box::new(DisplayReader::new(
            HidSource::new(0x24c0, 0x003).expect("could not start HID Api"),
        )),
        "RTL433" => Box::new(RTL433Reader::new(rtl_433_source())),
        _ => panic!("no reader defined"),
    }
}

fn find_writer(value: &str) -> AppWriter {
    match value {
        "INFLUXDB" => AppWriter::Influx(InfluxWriter::new()),
        "INMEMORY" => AppWriter::InMemory(InMemWriter::new()),
        "STDOUT" => AppWriter::Stdout(StdoutWriter::new()),
        _ => panic!("no writer defined"),
    }
}
