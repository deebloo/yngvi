use chrono::Utc;
use tokio::time::{sleep, Duration};

use acurite_core::formulas::{calc_dew_point, calc_heat_index, calc_wind_chill};
use acurite_core::{wind_dir_to_cardinal, Reader, RetryManager, WeatherReading, Writer};

use crate::decode::{
    decode_flavor, decode_out_humidity, decode_out_temp, decode_rain, decode_wind_dir,
    decode_wind_speed, validate_r1, Report1,
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

    // Open device and start reading reports.
    // If a failure to read occurs wait and then re-open device
    pub async fn start(&mut self, reader: &mut impl Reader<[u8; 10]>, writer: &mut impl Writer) {
        loop {
            self.run(reader, writer).await; // Run read write cycle

            sleep(Duration::from_secs(18)).await; // wait 18s for the next cycle
        }
    }

    // Run the read and write cycle once
    pub async fn run(&mut self, reader: &mut impl Reader<[u8; 10]>, writer: &mut impl Writer) {
        let mut buf: Report1 = [1u8; 10];

        if reader.read(&mut buf).is_ok() {
            if validate_r1(&buf) {
                self.weather_reading.time = Utc::now();

                self.update_weather_reading_r1(buf);

                let write_result = writer.write(&self.weather_reading).await;

                if write_result.is_ok() {
                    println!("{}", self.weather_reading);

                    self.retry_manager.replay_failed_writes(writer).await;
                } else {
                    println!("There was a problem when calling writer.write()");

                    self.retry_manager.add(self.weather_reading.clone());
                }
            } else {
                println!("Report R1 Invalid {:?}", buf);
            }
        } else {
            println!("Problem reading from device");
        }
    }

    fn update_weather_reading_r1(&mut self, data: Report1) {
        let report_flavor = decode_flavor(&data);

        // Both flavors have wind speed
        let wind_speed = decode_wind_speed(&data);
        self.weather_reading.wind_speed = Some(wind_speed);

        // Always clear rain_delta. (Will reassign if available)
        self.weather_reading.rain_delta = None;

        match report_flavor {
            1 => {
                // 2. Rain
                // 3. Wind Direction

                let new_rain_total = decode_rain(&data);

                // Update the rain delta if the new rain total is greater then the previously recorded
                if let Some(prev_rain_total) = self.weather_reading.rain {
                    if new_rain_total >= prev_rain_total {
                        self.weather_reading.rain_delta = Some(new_rain_total - prev_rain_total);
                    }
                }

                // Calculate wind chill if a temp has already been recorded
                if let Some(out_temp) = self.weather_reading.out_temp {
                    self.weather_reading.wind_chill = Some(calc_wind_chill(wind_speed, out_temp));
                }

                self.weather_reading.rain = Some(new_rain_total);

                let wind_dir = decode_wind_dir(&data);
                let wind_dir_cardinal = wind_dir_to_cardinal(wind_dir);

                self.weather_reading.wind_dir = Some(wind_dir);
                self.weather_reading.wind_dir_cardinal = Some(wind_dir_cardinal);
            }
            8 => {
                // 2. Outdoor temp
                // 3. Outdoor humidity

                let out_temp = decode_out_temp(&data);
                let out_humid = decode_out_humidity(&data);

                self.weather_reading.out_temp = Some(out_temp);
                self.weather_reading.out_humid = Some(out_humid);
                self.weather_reading.wind_chill = Some(calc_wind_chill(wind_speed, out_temp));
                self.weather_reading.heat_index = Some(calc_heat_index(out_temp, out_humid));
                self.weather_reading.dew_point = Some(calc_dew_point(out_temp, out_humid))
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_correct_reading() {
        let mut station = Station::new();

        station.update_weather_reading_r1([1, 197, 26, 120, 0, 5, 75, 75, 3, 255]);

        assert_eq!(
            station.weather_reading,
            WeatherReading {
                device_id: None,
                time: station.weather_reading.time,
                rain: None,
                rain_delta: None,
                wind_speed: Some(0.0),
                wind_dir: None,
                wind_dir_cardinal: None,
                out_temp: Some(31.499998),
                out_humid: Some(75),
                wind_chill: Some(31.499998),
                heat_index: Some(31.499998),
                dew_point: Some(24.52832)
            }
        )
    }

    #[test]
    fn calculates_rain_delta() {
        let mut station = Station::new();

        // rain total = 1.08
        station.update_weather_reading_r1([1, 197, 26, 113, 0, 200, 0, 108, 3, 255]);

        // rain total = 2.3600001
        station.update_weather_reading_r1([1, 197, 26, 113, 0, 200, 1, 108, 3, 255]);

        assert_eq!(station.weather_reading.rain_delta, Some(1.2800001))
    }
}
