use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    process::{Command, Stdio},
};
use ws_core::WeatherReading;

use crate::{BaseReading, FiveInOneReading};

pub fn rtl_433_source() -> impl Iterator<Item = Result<String>> {
    let stdout = Command::new("sh")
        .arg("-c")
        .arg("rtl_433 -C customary -F json -M time:iso:tz")
        .stdout(Stdio::piped())
        .spawn()
        .expect("COuld not spawn rtl_433 process")
        .stdout
        .expect("Could not capture standard output.");

    BufReader::new(stdout).lines()
}

pub fn rtl_433_file_source(path: &str) -> impl Iterator<Item = Result<String>> {
    let f = File::open(path).expect(format!("could not find file at {}", path).as_str());

    BufReader::new(f).lines()
}

pub struct RTL433Reader {}

impl RTL433Reader {
    pub fn new<T: Iterator<Item = Result<String>>>(
        source: T,
    ) -> impl Iterator<Item = WeatherReading> {
        source.filter_map(|line| line.ok()).filter_map(|line| {
            if let Ok(reading) = BaseReading::from_string(&line) {
                if reading.model == "Acurite-5n1" {
                    // parse the full 5n1 message
                    if let Ok(data) = FiveInOneReading::from_string(&line) {
                        if data.sequence_num == 0 {
                            let mut weather_reading = WeatherReading::new();

                            weather_reading.device_id = Some(data.id);
                            weather_reading.time = data.time;
                            weather_reading.rain = data.rain_in;
                            weather_reading.wind_speed = Some(data.wind_avg_mi_h);
                            weather_reading.out_temp = data.temperature_f;
                            weather_reading.out_humid = data.humidity;
                            weather_reading.wind_dir = data.wind_dir_deg;

                            return Some(weather_reading);
                        }
                    }
                }
            }

            None
        })
    }
}
