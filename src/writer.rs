use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait Writer {
    async fn write(&self, weather_reading: &WeatherReading) -> Result<(), ()>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherReading {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub rain_delta: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_dir: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
    pub heat_index: Option<f32>,
}

impl WeatherReading {
    pub fn new() -> WeatherReading {
        WeatherReading {
            time: Utc::now(),
            rain: None,
            rain_delta: None,
            wind_speed: None,
            wind_dir: None,
            out_temp: None,
            out_humid: None,
            wind_chill: None,
            heat_index: None,
        }
    }
}
