use crate::station::WeatherRecord;

use chrono::{DateTime, Utc};
use influxdb::{Client, Error, InfluxDbWriteable, Timestamp};

#[derive(InfluxDbWriteable)]
pub struct WeatherReadingType1 {
    time: DateTime<Utc>,
    rain: f32,
    wind_speed: f32,
    wind_chill: Option<f32>,
}

#[derive(InfluxDbWriteable)]
pub struct WeatherReadingType2 {
    time: DateTime<Utc>,
    wind_speed: f32,
    out_temp: f32,
    out_humid: u8,
    wind_chill: f32,
}

pub struct Writer<'a> {
    client: &'a Client,
}
impl<'a> Writer<'a> {
    pub fn new(client: &'a Client) -> Writer {
        Writer { client }
    }

    pub async fn write(&self, record: &WeatherRecord) -> Result<String, Error> {
        let time = Timestamp::from(Utc::now()).into();

        match record {
            WeatherRecord::Type1(value) => {
                let weather_reading = WeatherReadingType1 {
                    time,
                    rain: value.rain,
                    wind_speed: value.wind_speed,
                    wind_chill: value.wind_chill,
                };

                self.client
                    .query(&weather_reading.into_query("weather"))
                    .await
            }

            WeatherRecord::Type2(value) => {
                let weather_reading = WeatherReadingType2 {
                    time,
                    wind_speed: value.wind_speed,
                    out_temp: value.out_temp,
                    out_humid: value.out_humid,
                    wind_chill: value.wind_chill,
                };

                self.client
                    .query(&weather_reading.into_query("weather"))
                    .await
            }
        }
    }
}
