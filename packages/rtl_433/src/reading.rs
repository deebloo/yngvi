use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BaseReading {
    pub model: String,
}

impl BaseReading {
    pub fn from_string(buf: &String) -> Result<BaseReading, serde_json::Error> {
        serde_json::from_str::<BaseReading>(buf.as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FiveInOneReading {
    pub time: DateTime<Utc>,
    pub model: String,
    pub message_type: u8,
    pub id: u32,
    pub channel: String,
    pub sequence_num: u8,
    pub battery_ok: u8,
    pub mic: String,
    pub wind_avg_mi_h: f64,
    #[serde(rename(deserialize = "temperature_F"))]
    pub temperature_f: Option<f64>,
    pub humidity: Option<u8>,
    pub wind_dir_deg: Option<f64>,
    pub rain_in: Option<f64>,
}

impl FiveInOneReading {
    pub fn from_string(buf: &String) -> Result<FiveInOneReading, serde_json::Error> {
        serde_json::from_str::<FiveInOneReading>(buf.as_str())
    }
}
