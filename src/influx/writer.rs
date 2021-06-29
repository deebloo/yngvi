use async_trait::async_trait;
use influxdb::{Client, InfluxDbWriteable};
use serde::Deserialize;

use crate::config;
use crate::influx::WeatherReadingInflux;
use crate::writer::{WeatherReading, Writer};

#[derive(Deserialize, Debug, Clone)]
pub struct InfluxConfig {
    pub influx_addr: Option<String>,
    pub influx_db: Option<String>,
}

pub struct InfluxWriter {
    client: Client,

    #[allow(dead_code)]
    config: InfluxConfig,
}

impl InfluxWriter {
    pub fn new() -> Self {
        let config = Self::read_config();

        let client = Client::new(
            config.influx_addr.as_ref().unwrap(),
            config.influx_db.as_ref().unwrap(),
        );

        println!("{:?}", config);

        Self { client, config }
    }

    fn read_config() -> InfluxConfig {
        let mut config = config::read_config::<InfluxConfig>().unwrap_or(InfluxConfig {
            influx_addr: None,
            influx_db: None,
        });

        if config.influx_addr.is_none() {
            config.influx_addr = Some("http://localhost:8086".to_string());
        }

        if config.influx_db.is_none() {
            config.influx_db = Some("weather".to_string());
        }

        config
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
