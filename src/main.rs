use dotenv::dotenv;
use std::env;
use ws_core::{
    FileReader, InMemWriter, NoopWriter, Station, StdoutWriter, WeatherReadingSource, Writer,
};
use ws_display::{DisplayReader, HidSource};
use ws_influxdb::{Influx2Writer, InfluxWriter};
use ws_rtl_433::{rtl_433_source, RTL433Reader};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let source = var("SRC").unwrap_or("ACURITE_DISPLAY".to_string());
    let dest = var("DEST").unwrap_or("STDOUT".to_string());

    let mut station = Station::new();
    let reader = find_reader(&source);
    let mut writer = find_writer(&dest);

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    station.start(reader, &mut writer).await;
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

fn find_writer(value: &String) -> Box<dyn Writer + Send> {
    match value.to_uppercase().as_str() {
        "INFLUXDB" => Box::new(create_influx_writer()),
        "INFLUXDB2" => Box::new(create_influx2_writer()),
        "INMEMORY" => Box::new(InMemWriter::new()),
        "STDOUT" => Box::new(StdoutWriter::new()),
        "NOOP" => Box::new(NoopWriter::new()),
        _ => panic!("no writer defined. found {}", value),
    }
}

fn create_influx_writer() -> InfluxWriter {
    let url = var("DEST_INFLUXDB_URL").unwrap_or("http://localhost:8086".to_string());
    let database = var("DEST_INFLUXDB_DB").unwrap_or("weather".to_string());

    InfluxWriter::new(url, database)
}

fn create_influx2_writer() -> Influx2Writer {
    let url = var("DEST_INFLUXDB_URL").unwrap_or("http://localhost:8086".to_string());
    let org = var("DEST_INFLUXDB2_ORG").expect("ORG not provided");
    let bucket = var("DEST_INFLUXDB2_BUCKET").expect("BUCKET not provided");
    let token = var("DEST_INFLUXDB2_TOKEN").expect("TOKEN not provided");

    Influx2Writer::new(url, org, bucket, token)
}

fn var(key: &str) -> Result<String, env::VarError> {
    env::var(format!("WS_{}", key))
}
