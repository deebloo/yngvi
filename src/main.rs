use display::{DisplayReader, HidSource};
use influx_db::InfluxWriter;
use rtl_433::{rtl_433_source, RTL433Reader};
use std::env;
use weather::{InMemWriter, Station, StdoutWriter, WeatherReading};

#[tokio::main]
async fn main() {
    let source: String = env::var("WEATHER_SOURCE").unwrap_or("CONSOLE".to_string());
    let dest: String = env::var("WEATHER_DEST").unwrap_or("INFLUXDB".to_string());

    let mut station = Station::new();

    let reader: Box<dyn Iterator<Item = WeatherReading>> = match source.to_uppercase().as_str() {
        "CONSOLE" => Box::new(DisplayReader::new(
            HidSource::new(0x24c0, 0x003).expect("could not start HID Api"),
        )),
        "RTL433" => Box::new(RTL433Reader::new(rtl_433_source())),
        _ => panic!("no reader defined"),
    };

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    match dest.to_uppercase().as_str() {
        "INFLUXDB" => station.start(reader, &mut InfluxWriter::new()).await,
        "INMEMORY" => station.start(reader, &mut InMemWriter::new()).await,
        "STDOUT" => station.start(reader, &mut StdoutWriter::new()).await,
        _ => panic!("no writer defined"),
    };
}
