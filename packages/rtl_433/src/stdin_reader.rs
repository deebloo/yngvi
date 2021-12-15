use acurite_core::{ReadError, ReadResult, Reader};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BaseReading {
    pub model: String,
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
    pub wind_avg_mi_h: f32,
    #[serde(rename(deserialize = "temperature_F"))]
    pub temperature_f: Option<f32>,
    pub humidity: Option<u8>,
    pub wind_dir_deg: Option<f32>,
    pub rain_in: Option<f32>,
}

pub struct StdinReader {}

impl StdinReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Reader<String> for StdinReader {
    fn read(&mut self, buf: &mut String) -> ReadResult {
        if let Ok(_) = std::io::stdin().read_line(buf) {
            Ok(())
        } else {
            Err(ReadError::NoDevice)
        }
    }
}
