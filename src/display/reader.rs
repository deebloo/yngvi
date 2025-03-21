use crate::{
    core::WeatherReadingSource,
    display::decode::{
        decode_flavor, decode_out_humidity, decode_out_temp, decode_rain, decode_wind_dir,
        decode_wind_speed, Report1,
    },
};
use chrono::Utc;
use hidapi::HidApi;
use std::{thread, time};

pub struct HidSource {
    hid: HidApi,
    vid: u16,
    pid: u16,
}

impl HidSource {
    pub fn new(vid: u16, pid: u16) -> Result<Self, ()> {
        if let Ok(hid) = HidApi::new() {
            Ok(Self { hid, vid, pid })
        } else {
            Err(())
        }
    }
}

impl Iterator for HidSource {
    type Item = [u8; 10];

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Report1 = [1u8; 10];

        // we want to read every 18 seconds
        thread::sleep(time::Duration::from_secs(18));

        if let Ok(device) = self.hid.open(self.vid, self.pid) {
            match device.get_feature_report(&mut buf) {
                Ok(_) => Some(buf),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub struct DisplayReader;

impl DisplayReader {
    pub fn read_from<T: Iterator<Item = [u8; 10]>>(
        source: T,
    ) -> impl Iterator<Item = WeatherReadingSource> {
        source.map(|data| {
            let mut weather_reading = WeatherReadingSource::new();

            let report_flavor = decode_flavor(&data);

            // mark time as now
            weather_reading.time = Utc::now();

            // Both flavors have wind speed
            weather_reading.wind_speed = Some(decode_wind_speed(&data));

            match report_flavor {
                1 => {
                    // 2. Rain
                    // 3. Wind Direction

                    weather_reading.rain = Some(decode_rain(&data));
                    weather_reading.wind_dir = Some(decode_wind_dir(&data));
                }
                8 => {
                    // 2. Outdoor temp
                    // 3. Outdoor humidity

                    weather_reading.out_temp = Some(decode_out_temp(&data));
                    weather_reading.out_humid = Some(decode_out_humidity(&data));
                }
                _ => {}
            }

            weather_reading
        })
    }
}
