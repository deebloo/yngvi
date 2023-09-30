use crate::core::{
    calc_dew_point, calc_heat_index, calc_wind_chill, wind_dir_to_cardinal, RetryManager,
    WeatherReading, WeatherReadingSource, Writer,
};

pub struct Station {
    pub weather_reading: WeatherReading,
    pub retry_manager: RetryManager,
}

impl Station {
    pub fn new() -> Self {
        Self {
            weather_reading: WeatherReading::new(),
            retry_manager: RetryManager::new(),
        }
    }

    pub async fn start<R: Iterator<Item = WeatherReadingSource>, W: Writer>(
        &mut self,
        reader: R,
        writer: &mut W,
    ) {
        for reading in reader {
            self.weather_reading.device_id = reading.device_id;
            self.weather_reading.time = reading.time;
            self.weather_reading.wind_speed = reading.wind_speed;

            // be sure not to clear temp
            if let Some(out_temp) = reading.out_temp {
                self.weather_reading.out_temp = Some(out_temp);
            }

            // Calculate wind chill if a temp has already been recorded
            if let Some(out_temp) = self.weather_reading.out_temp {
                if let Some(wind_speed) = reading.wind_speed {
                    self.weather_reading.wind_chill = Some(calc_wind_chill(wind_speed, out_temp));
                }
            }

            // update humidity
            if let Some(out_humid) = reading.out_humid {
                self.weather_reading.out_humid = Some(out_humid);

                // update heat index and dew point
                if let Some(out_temp) = reading.out_temp {
                    self.weather_reading.heat_index = Some(calc_heat_index(out_temp, out_humid));
                    self.weather_reading.dew_point = Some(calc_dew_point(out_temp, out_humid));
                }
            }

            // update rain totals
            self.weather_reading.rain_delta = Some(0.0); // Always clear rain_delta. (Will reassign if available)

            // update rain totals
            if let Some(new_rain_total) = reading.rain {
                // Update the rain delta if the new rain total is greater then the previously recorded
                if let Some(prev_rain_total) = self.weather_reading.rain {
                    if new_rain_total >= prev_rain_total {
                        self.weather_reading.rain_delta = Some(new_rain_total - prev_rain_total);
                    }
                }

                self.weather_reading.rain = Some(new_rain_total);
            }

            // update wind direction
            if let Some(wind_direction) = reading.wind_dir {
                self.weather_reading.wind_dir = Some(wind_direction);

                let wind_dir_cardinal = wind_dir_to_cardinal(wind_direction);
                self.weather_reading.wind_dir_cardinal = Some(wind_dir_cardinal.to_string())
            }

            // write the result to the database
            let write_result = writer.write(&self.weather_reading).await;

            if write_result.is_ok() {
                self.retry_manager.replay_failed_writes(writer).await;
            } else {
                println!("There was a problem when calling writer.write()");

                self.retry_manager.add(self.weather_reading.clone());
            }
        }
    }
}
