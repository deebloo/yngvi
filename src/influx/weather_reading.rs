use chrono::{DateTime, Utc};
use influxdb::{InfluxDbWriteable, Timestamp};

use crate::writer::WeatherReading;

#[derive(InfluxDbWriteable, Clone, Copy, Debug, PartialEq)]
pub struct WeatherReadingInflux {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub rain_delta: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_dir: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
    pub heat_index: Option<f32>,
    pub dew_point: Option<f32>,
}

impl WeatherReadingInflux {
    pub fn from_weather_reading(weather_reading: &WeatherReading) -> WeatherReadingInflux {
        WeatherReadingInflux {
            time: Timestamp::from(weather_reading.time).into(),
            rain: weather_reading.rain,
            rain_delta: weather_reading.rain_delta,
            wind_speed: weather_reading.wind_speed,
            wind_dir: weather_reading.wind_dir,
            out_temp: weather_reading.out_temp,
            out_humid: weather_reading.out_humid,
            wind_chill: weather_reading.wind_chill,
            heat_index: weather_reading.heat_index,
            dew_point: weather_reading.dew_point
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_maps_to_influx_reading() {
        let time = Utc::now();
        let timestamp = Timestamp::from(time).into();

        let reading = WeatherReadingInflux::from_weather_reading(&WeatherReading {
            time,
            rain: Some(1.0),
            rain_delta: Some(0.1),
            wind_speed: Some(1.0),
            wind_dir: Some(270.0),
            out_temp: Some(80.0),
            out_humid: Some(50),
            wind_chill: Some(70.0),
            heat_index: Some(90.0),
            dew_point: None
        });

        assert_eq!(
            reading,
            WeatherReadingInflux {
                time: timestamp,
                rain: Some(1.0),
                rain_delta: Some(0.1),
                wind_speed: Some(1.0),
                wind_dir: Some(270.0),
                out_temp: Some(80.0),
                out_humid: Some(50),
                wind_chill: Some(70.0),
                heat_index: Some(90.0),
                dew_point: None
            }
        );
    }
}
