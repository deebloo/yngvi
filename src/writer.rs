use async_trait::async_trait;
use chrono::{DateTime, Utc};
use influxdb::{Client, Error, InfluxDbWriteable, Timestamp};
use std::env;

pub fn create_timestamp() -> DateTime<Utc> {
    Timestamp::from(Utc::now()).into()
}

#[async_trait]
pub trait Writer {
    async fn write(&self, weather_reading: &WeatherReading) -> Result<String, Error>;
}

#[derive(InfluxDbWriteable, Clone, Copy, Debug)]
pub struct WeatherReading {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub wind_speed: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
}

impl WeatherReading {
    pub fn new() -> WeatherReading {
        WeatherReading {
            time: create_timestamp(),
            rain: None,
            wind_speed: None,
            out_temp: None,
            out_humid: None,
            wind_chill: None,
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
    async fn write(&self, weather_reading: &WeatherReading) -> Result<String, Error> {
        let query = weather_reading.into_query("weather");

        self.client.query(&query).await
    }
}
