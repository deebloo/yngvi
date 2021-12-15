use chrono::DateTime;

use acurite_core::formulas::{calc_dew_point, calc_heat_index, calc_wind_chill};

use crate::rtl_433;
use acurite_core::writer::{WeatherReading, Writer};

pub struct Station {
    pub weather_reading: WeatherReading,
    pub failed_writes: Vec<WeatherReading>,
}

impl Station {
    pub fn new() -> Self {
        Self {
            weather_reading: WeatherReading::new(),
            failed_writes: vec![],
        }
    }

    // Open device and start reading reports.
    // If a failure to read occurs wait and then re-open device
    pub async fn start(&mut self, writer: &mut impl Writer) -> std::io::Result<()> {
        let mut count = 0;

        loop {
            count = count + 1;

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;

            let reading: rtl_433::WeatherReading = serde_json::from_str(buffer.as_str())?;

            if reading.sequence_num == 2 {
                self.update_weather_reading(&reading);

                let write_result = writer.write(&self.weather_reading).await;

                if write_result.is_ok() {
                    println!("{}", self.weather_reading);

                    self.replay_failed_writes(writer).await;
                } else {
                    println!("There was a problem when calling writer.write()");

                    self.failed_writes.push(self.weather_reading.clone());
                }
            }
        }
    }

    fn update_weather_reading(&mut self, data: &rtl_433::WeatherReading) {
        let time = DateTime::parse_from_rfc2822(data.time.as_str()).unwrap();

        self.weather_reading.time = DateTime::from(time);

        // Both flavors have wind speed
        self.weather_reading.wind_speed = Some(data.wind_avg_mi_h);

        // Always clear rain_delta. (Will reassign if available)
        self.weather_reading.rain_delta = None;

        // Update temp and wind chill
        if let Some(out_temp) = data.temperature_f {
            self.weather_reading.out_temp = Some(out_temp);
        }

        // Calculate wind chill if a temp has already been recorded
        if let Some(out_temp) = self.weather_reading.out_temp {
            self.weather_reading.wind_chill = Some(calc_wind_chill(data.wind_avg_mi_h, out_temp));
        }

        // update humidity
        if let Some(out_humid) = data.humidity {
            self.weather_reading.out_humid = Some(out_humid);

            // update heat index and dew point
            if let Some(out_temp) = data.temperature_f {
                self.weather_reading.heat_index = Some(calc_heat_index(out_temp, out_humid));
                self.weather_reading.dew_point = Some(calc_dew_point(out_temp, out_humid))
            }
        }

        // update rain totals
        if let Some(new_rain_total) = data.rain_in {
            self.weather_reading.rain = Some(new_rain_total);

            // Update the rain delta if the new rain total is greater then the previously recorded
            if let Some(prev_rain_total) = self.weather_reading.rain {
                if new_rain_total >= prev_rain_total {
                    self.weather_reading.rain_delta = Some(new_rain_total - prev_rain_total);
                }
            }
        }

        // update wind direction
        if let Some(wind_direction) = data.wind_dir_deg {
            self.weather_reading.wind_dir = Some(wind_direction);
        }
    }

    async fn replay_failed_writes(&mut self, writer: &mut impl Writer) {
        if self.failed_writes.len() > 0 {
            println!("Replaying previously failed writes");

            let mut writes_to_clear: Vec<WeatherReading> = vec![];

            for r in &self.failed_writes {
                let res = writer.write(r).await;

                if res.is_ok() {
                    writes_to_clear.push(r.clone());
                }
            }

            for r in writes_to_clear {
                self.failed_writes.retain(|x| x.time != r.time)
            }
        }
    }
}
