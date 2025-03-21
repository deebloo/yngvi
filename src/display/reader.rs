use crate::{
    core::WeatherReadingSource,
    display::decode::{
        decode_flavor, decode_out_humidity, decode_out_temp, decode_rain, decode_wind_dir,
        decode_wind_speed,
    },
};
use chrono::Utc;

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
