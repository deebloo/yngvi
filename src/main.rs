use std::env;
use ws_core::{FileReader, InMemWriter, NoopWriter, Station, StdoutWriter, WeatherReadingSource};
use ws_display::{DisplayReader, HidSource};
use ws_influx_db::InfluxWriter;
use ws_rtl_433::{rtl_433_source, RTL433Reader};

enum AppWriter {
    Influx(InfluxWriter),
    Stdout(StdoutWriter),
    InMemory(InMemWriter),
    Noop(NoopWriter),
}

#[tokio::main]
async fn main() {
    let source = env::var("WS_SOURCE").unwrap_or("DISPLAY".to_string());
    let dest = env::var("WS_DEST").unwrap_or("STDOUT".to_string());

    let mut station = Station::new();

    let reader = find_reader(&source);
    let mut writer = find_writer(&dest);

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    match &mut writer {
        AppWriter::Influx(writer) => station.start(reader, writer).await,
        AppWriter::InMemory(writer) => station.start(reader, writer).await,
        AppWriter::Stdout(writer) => station.start(reader, writer).await,
        AppWriter::Noop(writer) => station.start(reader, writer).await,
    }
}

fn find_reader(value: &String) -> Box<dyn Iterator<Item = WeatherReadingSource>> {
    match value.to_uppercase().as_str() {
        "DISPLAY" => Box::new(DisplayReader::new(
            HidSource::new(0x24c0, 0x003).expect("could not start HID Api"),
        )),
        "RTL433" => Box::new(RTL433Reader::new(rtl_433_source())),
        "FILE" => {
            let path = env::var("WS_SOURCE_FILE_PATH")
                .expect("WS_FILE_PATH is required when using the FILE source");

            Box::new(FileReader::new(path.as_str()))
        }
        _ => panic!("no reader defined. found {}", value),
    }
}

fn find_writer(value: &String) -> AppWriter {
    match value.to_uppercase().as_str() {
        "INFLUXDB" => AppWriter::Influx(InfluxWriter::new()),
        "INMEMORY" => AppWriter::InMemory(InMemWriter::new()),
        "STDOUT" => AppWriter::Stdout(StdoutWriter::new()),
        "NOOP" => AppWriter::Noop(NoopWriter::new()),
        _ => panic!("no writer defined. found {}", value),
    }
}
