use ws_core::WeatherReading;

pub trait LineProtocol {
    fn to_line_protocol(&self) -> String;
}

impl LineProtocol for WeatherReading {
    fn to_line_protocol(&self) -> String {
        let mut fields: Vec<String> = vec![];

        if let Some(value) = self.device_id {
            fields.push(format!("device_id={}i", value));
        }

        if let Some(value) = self.rain {
            fields.push(format!("rain={}", value));
        }

        if let Some(value) = self.rain_delta {
            fields.push(format!("rain_delta={}", value));
        }

        if let Some(value) = self.wind_speed {
            fields.push(format!("wind_speed={}", value));
        }

        if let Some(value) = self.wind_dir {
            fields.push(format!("wind_dir={}", value));
        }

        if let Some(value) = &self.wind_dir_cardinal {
            fields.push(format!("wind_dir_cardinal=\"{}\"", value));
        }

        if let Some(value) = self.out_temp {
            fields.push(format!("out_temp={}", value));
        }

        if let Some(value) = self.out_humid {
            fields.push(format!("out_humid={}i", value));
        }

        if let Some(value) = self.wind_chill {
            fields.push(format!("wind_chill={}", value));
        }

        if let Some(value) = self.heat_index {
            fields.push(format!("heat_index={}", value));
        }

        if let Some(value) = self.dew_point {
            fields.push(format!("dew_point={}", value));
        }

        format!(
            "weather {} {}",
            fields.join(","),
            self.time.timestamp_nanos()
        )
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

        assert_eq!(reading.to_line_protocol(), format!("weather device_id=100i,rain=100,rain_delta=0.5,wind_speed=4,wind_dir=180,wind_dir_cardinal=\"S\",out_temp=60.5,out_humid=50i,wind_chill=50,heat_index=60,dew_point=90 {}", reading.time.timestamp_nanos()));
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