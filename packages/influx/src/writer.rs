use acurite_core::{WeatherReading, Writer};
use async_trait::async_trait;
use influxdb::{Client, InfluxDbWriteable};
use std::env;

use crate::WeatherReadingInflux;

pub struct InfluxWriter {
    client: Client,
}

impl InfluxWriter {
    pub fn new() -> Self {
        let url = env::var("AR_INFLUXDB_URL").unwrap_or("http://localhost:8086".to_string());
        let database = env::var("AR_INFLUXDB_DB").unwrap_or("weather".to_string());

        let client = Client::new(url, database);

        Self { client }
    }
}

#[async_trait]
impl Writer for InfluxWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let weather_reading_influx = WeatherReadingInflux::from_weather_reading(&weather_reading);
        let query = weather_reading_influx.into_query("weather");

        let res = self.client.query(&query).await;

        if let Ok(_) = res {
            println!("Succssful write to Influxdb");

            Ok(())
        } else {
            Err(())
        }
    }
}
