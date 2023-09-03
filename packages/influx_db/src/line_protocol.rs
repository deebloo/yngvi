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
