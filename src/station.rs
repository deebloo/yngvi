use crate::util::calc_wind_chill;
use crate::writer::{create_timestamp, WeatherReading, Writer};

use async_std::task;
use hidapi::{HidApi, HidDevice};
use std::time::Duration;

type Report1 = [u8; 10];

pub struct DeviceIds {
    pub vid: u16,
    pub pid: u16,
}

pub struct Station<'a> {
    pub hid: &'a HidApi,
    pub writer: &'a Writer,
    pub device_ids: DeviceIds,
    weather_reading: WeatherReading,
    device: Option<HidDevice>,
}
impl<'a> Station<'a> {
    pub fn new(hid: &'a HidApi, device_ids: DeviceIds, writer: &'a Writer) -> Station<'a> {
        Station {
            hid,
            writer,
            device_ids,
            weather_reading: WeatherReading::new(),
            device: None,
        }
    }

    /**
     * Open device and start reading reports.
     * If a failure to read occurs wait and then re-open device
     */
    pub async fn start(&mut self) {
        self.open_device().await;

        loop {
            let report = self.read_report_r1();

            match report {
                Ok(report) => {
                    self.update_weather_reading_r1(report);

                    // write the result
                    let write_result = self.writer.write(&self.weather_reading).await;

                    if write_result.is_ok() {
                        println!("{:?}", self.weather_reading);
                    }

                    task::sleep(Duration::from_secs(18)).await;
                }
                Err(_) => {
                    println!("There was a problem reading report R1.");

                    task::sleep(Duration::from_secs(30)).await; // wait for a bit

                    self.open_device().await; // reopen device
                }
            }
        }
    }

    /**
     * Open HID device.
     * Attempt to connect 3 times
     */
    async fn open_device(&mut self) {
        let mut is_open = false;
        let mut retry_attempts = 0;
        let max_retry_attempts = 3;

        println!("Opening HID device...");

        while !is_open && retry_attempts <= max_retry_attempts {
            let open_result = self.hid.open(self.device_ids.vid, self.device_ids.pid);

            match open_result {
                Ok(device) => {
                    println!("HID device open...",);

                    is_open = true;

                    self.device = Some(device);
                }
                Err(_) => {
                    if retry_attempts == max_retry_attempts {
                        panic!(
                            "There was a problem opening the hid device. Retry attempts ({:?}) exceeded",
                            max_retry_attempts
                        );
                    } else {
                        retry_attempts += 1;

                        println!(
                            "There was a problem opening HID device. Retrying. Retry Attempt {:?}",
                            retry_attempts
                        );

                        task::sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        }
    }

    /**
     * Read and decode report R1
     */
    fn read_report_r1(&self) -> Result<Report1, &str> {
        if let Some(d) = &self.device {
            let mut buf: Report1 = [1u8; 10];

            let res = d.get_feature_report(&mut buf);

            match res {
                Ok(_) => Ok(buf),
                Err(_) => Err("Failed to read report"),
            }
        } else {
            Err("Failed to read report")
        }
    }

    fn update_weather_reading_r1(&mut self, data: Report1) {
        // update reading timestamp
        self.weather_reading.time = create_timestamp();

        let report_flavor = Station::decode_r1_flavor(&data);

        // Both flavors have wind speed
        let wind_speed = Station::decode_wind_speed(&data);
        self.weather_reading.wind_speed = Some(wind_speed);

        if report_flavor == 1 {
            self.weather_reading.rain = Some(Station::decode_rain(&data));

            if let Some(out_temp) = self.weather_reading.out_temp {
                // Calculate wind chill if a temp has already been recorded
                self.weather_reading.wind_chill = Some(calc_wind_chill(wind_speed, out_temp));
            }
        } else {
            let out_temp = Station::decode_out_temp(&data);

            self.weather_reading.out_temp = Some(out_temp);
            self.weather_reading.out_humid = Some(Station::decode_out_humidity(&data));
            self.weather_reading.wind_chill = Some(calc_wind_chill(wind_speed, out_temp));
        }
    }

    fn decode_wind_speed(data: &Report1) -> f32 {
        let n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

        if n == 0 {
            return 0.0;
        }

        (0.8278 * n as f32 + 1.0) / 1.609
    }

    fn decode_r1_flavor(data: &Report1) -> u8 {
        data[3] & 0x0f
    }

    fn decode_out_temp(data: &Report1) -> f32 {
        let a = ((data[5] & 0x0f) as u32) << 7;
        let b = (data[6] & 0x7f) as u32;
        let celcius = (a | b) as f32 / 18.0 - 40.0;

        (celcius * 9.) / 5. + 32.
    }

    fn decode_out_humidity(data: &Report1) -> u8 {
        data[7]
    }

    fn decode_rain(data: &Report1) -> f32 {
        let cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) as f32 * 0.0254;

        cm / 2.54
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_r1_falvor_1() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let flavor = Station::decode_r1_flavor(&report);

        assert_eq!(flavor, 1);
    }

    #[test]
    fn decode_r1_falvor_8() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let flavor = Station::decode_r1_flavor(&report);

        assert_eq!(flavor, 8);
    }

    #[test]
    fn decode_rain() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let rain = Station::decode_rain(&report);

        assert_eq!(rain, 1.08);
    }

    #[test]
    fn decode_wind_speed() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let wind_speed = Station::decode_wind_speed(&report);

        assert_eq!(wind_speed, 2.6794283);
    }

    #[test]
    fn decode_out_temp() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let out_temp = Station::decode_out_temp(&report);

        assert_eq!(out_temp, 31.499998);
    }

    #[test]
    fn decode_out_humid() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let out_humid = Station::decode_out_humidity(&report);

        assert_eq!(out_humid, 75);
    }
}
