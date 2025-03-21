use crate::{
    core::{WeatherReading, Writer},
    influxdb::line_protocol::LineProtocol,
};
use reqwest::Client;

pub struct InfluxWriter {
    url: String,
    database: String,
    client: Client,
}

impl InfluxWriter {
    pub fn new(url: String, database: String) -> Self {
        Self {
            url,
            database,
            client: Client::new(),
        }
    }
}

impl Writer for InfluxWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let query = weather_reading.to_line_protocol();

        let url = format!("{}/write", self.url);
        let request = self
            .client
            .post(url)
            .query(&[("db", &self.database), ("precision", &String::from("ms"))])
            .body(query);

        if let Ok(response) = request.send().await {
            if response.status() == 204 {
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
