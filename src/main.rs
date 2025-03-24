use dotenv::dotenv;

use yngvi::{
    core::{
        FileReader, InMemWriter, NoopWriter, Station, StdinReader, StdoutWriter,
        WeatherReadingSource, WebhookWriter, Writer,
    },
    display::{DisplayReader, HidSource},
    influxdb::InfluxWriter,
    rtl_433::{rtl_433_source, RTL433Reader},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let source = env_var("SRC").unwrap_or("ACURITE_DISPLAY".to_string());
    let dest = env_var("DEST").unwrap_or("STDOUT".to_string());

    let mut station = Station::new();
    let reader = find_reader(&source);
    let mut writer = find_writer(&dest);

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    let res = station.start(reader, &mut writer).await;

    if res.is_ok() {
        println!("Station no longer recieving readings. Shutting down");
    }
}

pub enum AppWriter {
    InfluxDB2(InfluxWriter),
    InMemory(InMemWriter),
    Stdout(StdoutWriter),
    Webhook(WebhookWriter),
    Noop(NoopWriter),
}

impl Writer for AppWriter {
    async fn write(&mut self, weather_reading: &yngvi::core::WeatherReading) -> Result<(), ()> {
        match self {
            AppWriter::InfluxDB2(writer) => writer.write(weather_reading).await,
            AppWriter::InMemory(writer) => writer.write(weather_reading).await,
            AppWriter::Stdout(writer) => writer.write(weather_reading).await,
            AppWriter::Webhook(writer) => writer.write(weather_reading).await,
            AppWriter::Noop(writer) => writer.write(weather_reading).await,
        }
    }
}

fn env_var(key: &str) -> Result<String, std::env::VarError> {
    std::env::var(format!("YNGVI_{}", key))
}

pub fn find_writer(value: &String) -> AppWriter {
    match value.to_uppercase().as_str() {
        "INFLUXDB" => {
            let url = env_var("DEST_INFLUXDB_URL").unwrap_or("http://localhost:8086".to_string());
            let org = env_var("DEST_INFLUXDB_ORG").expect("ORG not provided");
            let bucket = env_var("DEST_INFLUXDB_BUCKET").expect("BUCKET not provided");
            let token = env_var("DEST_INFLUXDB_TOKEN").expect("TOKEN not provided");

            AppWriter::InfluxDB2(InfluxWriter::new(url, org, bucket, token))
        }
        "INMEMORY" => {
            let mem = InMemWriter::new();

            AppWriter::InMemory(mem)
        }
        "STDOUT" => {
            let stdout = StdoutWriter::new();

            AppWriter::Stdout(stdout)
        }
        "WEBHOOK" => {
            let url = env_var("DEST_WEBHOOK_URL").expect("No url defined for webhook");
            let raw_headers = env_var("DEST_WEBHOOK_HEADERS").unwrap_or("".to_string());

            let mut headers: Vec<(String, String)> = vec![];

            for header_key_value in raw_headers.split(",") {
                let mut header = header_key_value.split(":");
                let key = header.next().expect("No Key found for header");
                let value = header.next().expect("No Value found for header");

                headers.push((key.to_string(), value.to_string()));
            }

            let webhook = WebhookWriter::new(url, headers);

            AppWriter::Webhook(webhook)
        }
        "NOOP" => {
            let noop = NoopWriter::new();

            AppWriter::Noop(noop)
        }
        _ => panic!("no writer defined. found {}", value),
    }
}

pub fn find_reader(value: &String) -> Box<dyn Iterator<Item = WeatherReadingSource>> {
    match value.to_uppercase().as_str() {
        "ACURITE_DISPLAY" => {
            let source = HidSource::new(0x24c0, 0x003).expect("could not start HID Api");

            Box::new(DisplayReader::read_from(source))
        }
        "RTL_433" => {
            let source = rtl_433_source();

            Box::new(RTL433Reader::read_from(source))
        }
        "FILE" => {
            let path = env_var("SRC_FILE_PATH").expect("PATH not provided");

            Box::new(FileReader::read_from(path.as_str()))
        }
        "STDIN" => {
            let reader = StdinReader::read();

            Box::new(reader)
        }
        _ => panic!("no reader defined. found {}", value),
    }
}
