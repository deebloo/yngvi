use std::env;
use ws_core::{FileReader, InMemWriter, NoopWriter, Station, StdoutWriter, WeatherReadingSource};
use ws_display::{DisplayReader, HidSource};
use ws_influxdb::{Influx2Writer, InfluxWriter};
use ws_rtl_433::{rtl_433_source, RTL433Reader};

enum AppWriter {
    Influx(InfluxWriter),
    Influx2(Influx2Writer),
    Stdout(StdoutWriter),
    InMemory(InMemWriter),
    Noop(NoopWriter),
}

#[tokio::main]
async fn main() {
    let source = env::var("WS_SRC").unwrap_or("ACURITE_DISPLAY".to_string());
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
        AppWriter::Influx2(writer) => station.start(reader, writer).await,
        AppWriter::InMemory(writer) => station.start(reader, writer).await,
        AppWriter::Stdout(writer) => station.start(reader, writer).await,
        AppWriter::Noop(writer) => station.start(reader, writer).await,
    }
}

fn find_reader(value: &String) -> Box<dyn Iterator<Item = WeatherReadingSource>> {
    match value.to_uppercase().as_str() {
        "ACURITE_DISPLAY" => {
            let source = HidSource::new(0x24c0, 0x003).expect("could not start HID Api");

            Box::new(DisplayReader::new(source))
        }
        "RTL_433" => {
            let source = rtl_433_source();

            Box::new(RTL433Reader::new(source))
        }
        "FILE" => {
            let path = env::var("WS_SRC_FILE_PATH").expect("PATH not provided");

            Box::new(FileReader::new(path.as_str()))
        }
        _ => panic!("no reader defined. found {}", value),
    }
}

fn find_writer(value: &String) -> AppWriter {
    match value.to_uppercase().as_str() {
        "INFLUXDB" => AppWriter::Influx(create_influx_writer()),
        "INFLUXDB2" => AppWriter::Influx2(create_influx2_writer()),
        "INMEMORY" => AppWriter::InMemory(InMemWriter::new()),
        "STDOUT" => AppWriter::Stdout(StdoutWriter::new()),
        "NOOP" => AppWriter::Noop(NoopWriter::new()),
        _ => panic!("no writer defined. found {}", value),
    }
}

fn create_influx_writer() -> InfluxWriter {
    let url_key = "WS_DEST_INFLUXDB_URL";
    let db_key = "WS_DEST_INFLUXDB_DB";

    let url = env::var(url_key).unwrap_or("http://localhost:8086".to_string());
    let database = env::var(db_key).unwrap_or("weather".to_string());

    InfluxWriter::new(url, database)
}

fn create_influx2_writer() -> Influx2Writer {
    let url_key = "WS_DEST_INFLUXDB_URL";
    let org_key = "WS_DEST_INFLUXDB2_ORG";
    let bucket_key = "WS_DEST_INFLUXDB2_BUCKET";
    let token_key = "WS_DEST_INFLUXDB2_TOKEN";

    let url = env::var(url_key).unwrap_or("http://localhost:8086".to_string());
    let org = env::var(org_key).expect("ORG not provided");
    let bucket = env::var(bucket_key).expect("BUCKET not provided");
    let token = env::var(token_key).expect("TOKEN not provided");

    Influx2Writer::new(url, org, bucket, token)
}
