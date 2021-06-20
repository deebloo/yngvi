use async_trait::async_trait;
use influxdb::{Client, InfluxDbWriteable};
use std::env;

use crate::influx::WeatherReadingInflux;
use crate::writer::{WeatherReading, Writer};

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
