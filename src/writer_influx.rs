use async_trait::async_trait;
use chrono::{DateTime, Utc};
use influxdb::{Client, InfluxDbWriteable, Timestamp};
use std::env;

use crate::writer::{WeatherReading, Writer};

#[derive(InfluxDbWriteable, Clone, Copy, Debug)]
pub struct WeatherReadingInflux {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub wind_speed: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
}

impl WeatherReadingInflux {
    pub fn from_weather_reading(weather_reading: &WeatherReading) -> WeatherReadingInflux {
        WeatherReadingInflux {
            time: Timestamp::from(weather_reading.time).into(),
            rain: weather_reading.rain,
            wind_speed: weather_reading.wind_speed,
            out_temp: weather_reading.out_temp,
            out_humid: weather_reading.out_humid,
            wind_chill: weather_reading.wind_chill,
        }
    }
}

pub struct InfluxWriter {
    client: Client,
}

impl InfluxWriter {
    pub fn new() -> InfluxWriter {
        let defaults_influx_addr = String::from("http://localhost:8086");
        let influx_addr = env::var("INFLUX_ADDR").unwrap_or(defaults_influx_addr);
        let client = Client::new(influx_addr, "weather");

        InfluxWriter { client }
    }
}

#[async_trait]
impl Writer for InfluxWriter {
    async fn write(&self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let weather_reading_influx = WeatherReadingInflux::from_weather_reading(&weather_reading);
        let query = weather_reading_influx.into_query("weather");

        let res = self.client.query(&query).await;

        if let Ok(_) = res {
            Ok(())
        } else {
            Err(())
        }
    }
}
