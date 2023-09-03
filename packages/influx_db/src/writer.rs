use async_trait::async_trait;
use reqwest::Client;
use ws_core::{WeatherReading, Writer};

use crate::line_protocol::LineProtocol;

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

#[async_trait]
impl Writer for InfluxWriter {
    async fn write(&mut self, weather_reading: &WeatherReading) -> Result<(), ()> {
        let query = weather_reading.to_line_protocol();

        let url = format!("{}/write", self.url);
        let request = self
            .client
            .post(url)
            .query(&[("db", &self.database)])
            .body(query)
            .send()
            .await;

        if let Ok(response) = request {
            println!("{:?}", response);

            if response.status() == 204 {
                println!("Write to InfluxDB Successful");
                println!("{:?}", weather_reading);

                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_line_protocol_0() {
        let reading = WeatherReading {
            device_id: Some(100),
            time: chrono::Utc::now(),
            rain: Some(100.),
            rain_delta: Some(0.5),
            wind_speed: Some(4.),
            wind_dir: Some(180.),
            wind_dir_cardinal: Some("S".to_string()),
            out_temp: Some(60.5),
            out_humid: Some(50),
            wind_chill: Some(50.),
            heat_index: Some(60.),
            dew_point: Some(90.),
        };

        assert_eq!(reading.to_line_protocol(), format!("weather device_id=100i,rain=100,rain_delta=0.5,wind_speed=4,wind_dir=180,wind_dir_cardinal=\"S\",out_temp=60.5,out_humid=50,wind_chill=50,heat_index=60,dew_point=90 {}", reading.time.timestamp_nanos()));
    }

    #[test]
    fn should_handle_empty_values() {
        let mut reading = WeatherReading::new();

        reading.out_temp = Some(60.);

        assert_eq!(
            reading.to_line_protocol(),
            format!("weather out_temp=60 {}", reading.time.timestamp_nanos())
        );
    }
}
