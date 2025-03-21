use yngvi::{
    core::{FileReader, InMemWriter, NoopWriter, StdoutWriter, WeatherReadingSource},
    display::{DisplayReader, HidSource},
    influxdb::{Influx2Writer, InfluxWriter},
    rtl_433::{rtl_433_source, RTL433Reader},
};

use crate::writers::AppWriter;

pub fn find_reader(value: &String) -> Box<dyn Iterator<Item = WeatherReadingSource>> {
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
            let path = var("SRC_FILE_PATH").expect("PATH not provided");

            Box::new(FileReader::new(path.as_str()))
        }
        _ => panic!("no reader defined. found {}", value),
    }
}

pub fn find_writer(value: &String) -> AppWriter {
    match value.to_uppercase().as_str() {
        "INFLUXDB" => AppWriter::InfluxDB(create_influx_writer()),
        "INFLUXDB2" => AppWriter::InfluxDB2(create_influx2_writer()),
        "INMEMORY" => AppWriter::InMemory(InMemWriter::new()),
        "STDOUT" => AppWriter::Stdout(StdoutWriter::new()),
        "NOOP" => AppWriter::Noop(NoopWriter::new()),
        _ => panic!("no writer defined. found {}", value),
    }
}

pub fn create_influx_writer() -> InfluxWriter {
    let url = var("DEST_INFLUXDB_URL").unwrap_or("http://localhost:8086".to_string());
    let database = var("DEST_INFLUXDB_DB").unwrap_or("weather".to_string());

    InfluxWriter::new(url, database)
}

pub fn create_influx2_writer() -> Influx2Writer {
    let url = var("DEST_INFLUXDB2_URL").unwrap_or("http://localhost:8086".to_string());
    let org = var("DEST_INFLUXDB2_ORG").expect("ORG not provided");
    let bucket = var("DEST_INFLUXDB2_BUCKET").expect("BUCKET not provided");
    let token = var("DEST_INFLUXDB2_TOKEN").expect("TOKEN not provided");

    Influx2Writer::new(url, org, bucket, token)
}

pub fn var(key: &str) -> Result<String, std::env::VarError> {
    std::env::var(format!("WS_{}", key))
}
