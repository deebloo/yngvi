use chrono::{DateTime, Utc};
use influxdb::{Client, Error, InfluxDbWriteable, Timestamp};

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
            time: Writer::create_timestamp(),
            rain: None,
            wind_speed: None,
            out_temp: None,
            out_humid: None,
            wind_chill: None,
        }
    }
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

    pub fn create_timestamp() -> DateTime<Utc> {
        Timestamp::from(Utc::now()).into()
    }
}
