use crate::core::WeatherReading;
use reqwest::Client;
use std::future::Future;

pub trait Writer {
    fn write(&mut self, weather_reading: &WeatherReading) -> impl Future<Output = Result<(), ()>>;
}

impl<T: Writer + ?Sized + Send> Writer for Box<T> {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        (**self).write(&weather_reading).await
    }
}

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Writer for StdoutWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        if let Ok(json) = serde_json::to_string(weather_reading) {
            println!("{}", json);
        }

        Ok(())
    }
}

pub struct InMemWriter {
    pub readings: Vec<WeatherReading>,
}

impl InMemWriter {
    pub fn new() -> Self {
        Self { readings: vec![] }
    }
}

impl Writer for InMemWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}

pub struct NoopWriter {}

impl NoopWriter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Writer for NoopWriter {
    async fn write(&mut self, _: &WeatherReading) -> Result<(), ()> {
        Ok(())
    }
}

pub struct WebhookWriter {
    pub url: String,
    pub client: Client,
    pub headers: Vec<(String, String)>,
}

impl WebhookWriter {
    pub fn new(url: String, headers: Vec<(String, String)>) -> Self {
        Self {
            url: url.to_string(),
            client: Client::new(),
            headers,
        }
    }
}

impl Writer for WebhookWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(weather_reading);

        for header in &self.headers {
            request = request.header(&header.0, &header.1);
        }

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
