use clap::Parser;
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

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    #[arg(long, default_value_t = String::from("acurite_display"))]
    src: String,

    #[arg(long, default_value_t = String::from("stdout"))]
    dest: String,

    #[arg(long, default_value_t = String::from("http://localhost:8086"))]
    influx_db_url: String,

    #[arg(long)]
    influx_db_org: Option<String>,

    #[arg(long)]
    influx_db_bucket: Option<String>,

    #[arg(long)]
    influx_db_token: Option<String>,

    #[arg(long)]
    webhook_url: Option<String>,

    #[arg(long)]
    webhook_headers: Option<String>,

    #[arg(long)]
    src_file_path: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let program = Program::parse();

    let mut station = Station::new();
    let reader = find_reader(&program);
    let mut writer = find_writer(&program);

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        program.src, program.dest
    );

    let res = station.start(reader, &mut writer).await;

    if res.is_ok() {
        println!("Station no longer recieving readings. Shutting down");
    }
}

pub enum AppWriter {
    InfluxDB(InfluxWriter),
    InMemory(InMemWriter),
    Stdout(StdoutWriter),
    Webhook(WebhookWriter),
    Noop(NoopWriter),
}

impl Writer for AppWriter {
    async fn write(&mut self, weather_reading: &yngvi::core::WeatherReading) -> Result<(), ()> {
        match self {
            AppWriter::InfluxDB(writer) => writer.write(weather_reading).await,
            AppWriter::InMemory(writer) => writer.write(weather_reading).await,
            AppWriter::Stdout(writer) => writer.write(weather_reading).await,
            AppWriter::Webhook(writer) => writer.write(weather_reading).await,
            AppWriter::Noop(writer) => writer.write(weather_reading).await,
        }
    }
}

fn find_writer(value: &Program) -> AppWriter {
    match value.dest.to_uppercase().as_str() {
        "INFLUXDB" => {
            let org = value
                .influx_db_org
                .clone()
                .expect("no influxdb org defined");

            let bucket = value
                .influx_db_bucket
                .clone()
                .expect("no influxdb bucket defined");

            let token = value
                .influx_db_bucket
                .clone()
                .expect("no influxdb token defined");

            AppWriter::InfluxDB(InfluxWriter::new(
                value.influx_db_url.clone(),
                org,
                bucket,
                token,
            ))
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
            let url = value
                .webhook_url
                .clone()
                .expect("No url defined for webhook");

            let raw_headers = value.webhook_headers.clone().unwrap_or("".to_string());

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
        _ => panic!("no writer defined. found {}", value.dest),
    }
}

fn find_reader(value: &Program) -> Box<dyn Iterator<Item = WeatherReadingSource>> {
    match value.src.to_uppercase().as_str() {
        "ACURITE_DISPLAY" => {
            let source = HidSource::new(0x24c0, 0x003).expect("could not start HID Api");

            Box::new(DisplayReader::read_from(source))
        }
        "RTL_433" => {
            let source = rtl_433_source();

            Box::new(RTL433Reader::read_from(source))
        }
        "FILE" => {
            let path = value.src_file_path.clone().expect("PATH not provided");

            Box::new(FileReader::read_from(path.as_str()))
        }
        "STDIN" => {
            let reader = StdinReader::read();

            Box::new(reader)
        }
        _ => panic!("no reader defined. found {}", value.src),
    }
}
