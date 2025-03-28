use crate::{
    core::WeatherReadingSource,
    rtl_433::{BaseReading, FiveInOneReading},
};
use metrum::Temp;
use std::{
    io::{BufRead, BufReader, Result},
    process::{Command, Stdio},
};

pub struct RTL433Reader {}

impl RTL433Reader {
    pub fn read() -> impl Iterator<Item = WeatherReadingSource> {
        let stdout = Command::new("sh")
            .arg("-c")
            .arg("rtl_433 -C customary -F json -M time:iso:tz")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn rtl_433 process")
            .stdout
            .expect("Could not capture standard output.");

        let source = BufReader::new(stdout).lines();

        Self::read_from(source)
    }

    pub fn read_from<T: Iterator<Item = Result<String>>>(
        source: T,
    ) -> impl Iterator<Item = WeatherReadingSource> {
        source.filter_map(|line| line.ok()).filter_map(|line| {
            if let Ok(reading) = BaseReading::from_string(&line) {
                if reading.model == "Acurite-5n1" {
                    // parse the full 5n1 message
                    if let Ok(data) = FiveInOneReading::from_string(&line) {
                        if data.sequence_num == 0 {
                            let mut weather_reading = WeatherReadingSource::new();

                            weather_reading.device_id = Some(data.id);
                            weather_reading.time = data.time;
                            weather_reading.rain = data.rain_in;
                            weather_reading.wind_speed = Some(data.wind_avg_mi_h);
                            weather_reading.out_temp = if let Some(temp) = data.temperature_f {
                                Some(Temp::from_f(temp))
                            } else {
                                None
                            };
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
