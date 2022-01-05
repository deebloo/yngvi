use crate::reading::{BaseReading, FiveInOneReading};
use acurite_core::formulas::{calc_dew_point, calc_heat_index, calc_wind_chill};
use acurite_core::{Reader, RetryManager, WeatherReading, Writer};

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
    pub async fn start(&mut self, reader: &mut impl Reader<String>, writer: &mut impl Writer) {
        loop {
            self.run(reader, writer).await;
        }
    }

    pub async fn run(&mut self, reader: &mut impl Reader<String>, writer: &mut impl Writer) {
        let mut buf = String::new();

        // make sure read is successful
        if reader.read(&mut buf).is_ok() {
            // parse the bare minimum to get the model
            if let Ok(reading) = BaseReading::from_string(&buf) {
                // make sure the model is the 5n1
                // TODO: Update json parser to more elegantly handle other acurite stations (mainly atlas)
                if reading.model == "Acurite-5n1" {
                    // parse the full 5n1 message
                    if let Ok(five_n_one) = FiveInOneReading::from_string(&buf) {
                        // the message will come in 3 things (0 base indexed) only grab the last one
                        if five_n_one.sequence_num == 0 {
                            // update the weather reading in place
                            self.update_weather_reading(&five_n_one);

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
            } else {
                println!("### There was an Error Parsing the following message ###");
                println!("{}", &buf);
            }
        }
    }

    fn update_weather_reading(&mut self, data: &FiveInOneReading) {
        self.weather_reading.device_id = Some(data.id);
        self.weather_reading.time = data.time;

        // Both flavors have wind speed
        self.weather_reading.wind_speed = Some(data.wind_avg_mi_h);

        // Update temp and wind chill
        if let Some(out_temp) = data.temperature_f {
            self.weather_reading.out_temp = Some(out_temp);
        }

        // Calculate wind chill if a temp has already been recorded
        if let Some(out_temp) = self.weather_reading.out_temp {
            self.weather_reading.wind_chill = Some(calc_wind_chill(data.wind_avg_mi_h, out_temp));
        }

        // update humidity
        self.update_humidity(&data);

        // update rain totals
        self.update_rain_totals(&data);

        // update wind direction
        if let Some(wind_direction) = data.wind_dir_deg {
            self.weather_reading.wind_dir = Some(wind_direction);
        }
    }

    fn update_humidity(&mut self, data: &FiveInOneReading) {
        if let Some(out_humid) = data.humidity {
            self.weather_reading.out_humid = Some(out_humid);

            // update heat index and dew point
            if let Some(out_temp) = data.temperature_f {
                self.weather_reading.heat_index = Some(calc_heat_index(out_temp, out_humid));
                self.weather_reading.dew_point = Some(calc_dew_point(out_temp, out_humid));
            }
        }
    }

    fn update_rain_totals(&mut self, data: &FiveInOneReading) {
        // Always clear rain_delta. (Will reassign if available)
        self.weather_reading.rain_delta = Some(0.0);

        // update rain totals
        if let Some(new_rain_total) = data.rain_in {
            // Update the rain delta if the new rain total is greater then the previously recorded
            if let Some(prev_rain_total) = self.weather_reading.rain {
                if new_rain_total >= prev_rain_total {
                    self.weather_reading.rain_delta = Some(new_rain_total - prev_rain_total);
                }
            }

            self.weather_reading.rain = Some(new_rain_total);
        }
    }
}
