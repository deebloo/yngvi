use chrono::{DateTime, Utc};
use influxdb::{Client, Error, InfluxDbWriteable};

#[derive(InfluxDbWriteable, Clone, Copy, Debug)]
pub struct WeatherReading {
    pub time: DateTime<Utc>,
    pub rain: Option<f32>,
    pub wind_speed: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
}

pub struct Writer<'a> {
    client: &'a Client,
}
impl<'a> Writer<'a> {
    pub fn new(client: &'a Client) -> Writer {
        Writer { client }
    }

    pub async fn write(&self, weather_reading: &WeatherReading) -> Result<String, Error> {
        let query = weather_reading.into_query("weather");

        self.client.query(&query).await
    }
}
