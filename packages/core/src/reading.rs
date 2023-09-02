use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherReadingSource {
    pub time: DateTime<Utc>,
    pub device_id: Option<u32>,
    pub rain: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_dir: Option<f32>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
}

impl WeatherReadingSource {
    pub fn new() -> Self {
        Self {
            device_id: None,
            time: Utc::now(),
            rain: None,
            wind_speed: None,
            wind_dir: None,
            out_temp: None,
            out_humid: None,
        }
    }

    pub fn from_str(buf: &String) -> Result<WeatherReading, serde_json::Error> {
        serde_json::from_str::<WeatherReading>(buf.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WeatherReading {
    pub time: DateTime<Utc>,
    pub device_id: Option<u32>,
    pub rain: Option<f32>,
    pub rain_delta: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_dir: Option<f32>,
    pub wind_dir_cardinal: Option<String>,
    pub out_temp: Option<f32>,
    pub out_humid: Option<u8>,
    pub wind_chill: Option<f32>,
    pub heat_index: Option<f32>,
    pub dew_point: Option<f32>,
}

impl fmt::Display for WeatherReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#### {:?} #### \nrain: {:?} rain_delta: {:?} \nwind_speed: {:?} wind_dir: {:?} \nwind_dir_cardinal: {:?} \ntemp: {:?} humidity: {:?} \nwind_chill: {:?} heat_index: {:?} dew_point: {:?} \n###### END ######\n",
            self.time, self.rain, self.rain_delta, self.wind_speed, self.wind_dir, self.wind_dir_cardinal, self.out_temp, self.out_humid, self.wind_chill, self.heat_index, self.dew_point
        )
    }
}

impl WeatherReading {
    pub fn new() -> WeatherReading {
        WeatherReading {
            device_id: None,
            time: Utc::now(),
            rain: None,
            rain_delta: None,
            wind_speed: None,
            wind_dir: None,
            wind_dir_cardinal: None,
            out_temp: None,
            out_humid: None,
            wind_chill: None,
            heat_index: None,
            dew_point: None,
        }
    }
}
