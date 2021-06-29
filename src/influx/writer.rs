use async_trait::async_trait;
use influxdb::{Client, InfluxDbWriteable};
use serde::Deserialize;
use std::error::Error;

use crate::config;
use crate::influx::WeatherReadingInflux;
use crate::writer::{WeatherReading, Writer};

#[derive(Deserialize, Debug)]
pub struct InfluxConfig {
    pub influx_addr: Option<String>,
}

pub struct InfluxWriter {
    client: Client,
}

impl InfluxWriter {
    pub fn new() -> Self {
        let config = Self::read_config().unwrap_or(InfluxConfig {
            influx_addr: None
        });

        let influx_addr = config.influx_addr.unwrap_or(String::from("http://localhost:8086"));

        let client = Client::new(influx_addr, "weather");

        Self { client }
    }

    pub fn read_config() -> Result<InfluxConfig, Box<dyn Error>> {
        config::read_config::<InfluxConfig>()
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
