use crate::{
    core::{WeatherReading, Writer},
    influxdb::line_protocol::LineProtocol,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct Influx2Writer {
    url: String,
    org: String,
    bucket: String,
    token: String,
    client: Client,
}

impl Influx2Writer {
    pub fn new(url: String, org: String, bucket: String, token: String) -> Self {
        Self {
            url,
            org,
            bucket,
            token,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Writer for Influx2Writer {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let query = weather_reading.to_line_protocol();

        println!("{}", &query);

        let url = format!("{}/api/v2/write", self.url);
        let request = self
            .client
            .post(url)
            .query(&[
                ("org", &self.org),
                ("bucket", &self.bucket),
                ("precision", &String::from("ms")),
            ])
            .header("Authorization", format!("Token {}", self.token))
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
