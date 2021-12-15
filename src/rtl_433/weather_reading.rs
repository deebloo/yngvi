use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WeatherReading {
    time: String,
    model: String,
    message_type: u8,
    id: u32,
    channel: String,
    sequence_num: u8,
    battery_ok: u8,
    mic: String,
    wind_avg_mi_h: f32,
    #[serde(rename(serialize = "temperature_F"))]
    temperature_f: Option<f32>,
    humidity: Option<u32>,
    wind_dir_deg: Option<f32>,
    rain_in: Option<f32>,
}
