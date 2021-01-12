use crate::util::calc_wind_chill;
use crate::writer::{WeatherReading, Writer};

use chrono::Utc;
use hidapi::{HidApi, HidDevice};
use influxdb::Timestamp;
use settimeout::set_timeout;
use std::time::Duration;

type Report1 = [u8; 10];

#[derive(Debug, Clone, Copy)]
pub struct WeatherRecordType1 {
    pub wind_speed: f32,
    pub rain: f32,
    pub wind_chill: Option<f32>,
    pub out_temp: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct WeatherRecordType2 {
    pub wind_speed: f32,
    pub out_temp: f32,
    pub out_humid: u8,
    pub wind_chill: f32,
}

pub enum WeatherRecord {
    Type1(WeatherRecordType1),
    Type2(WeatherRecordType2),
}

pub struct DeviceIds {
    pub vid: u16,
    pub pid: u16,
}

pub struct Station<'a> {
    pub hid: &'a HidApi,
    pub writer: &'a Writer<'a>,
    pub device_ids: DeviceIds,
    last_recorded_temp: Option<f32>, // keep track of the last recorded temp
    device: Option<HidDevice>,
}
impl<'a> Station<'a> {
    pub fn new(hid: &'a HidApi, device_ids: DeviceIds, writer: &'a Writer<'a>) -> Station<'a> {
        Station {
            hid,
            writer,
            last_recorded_temp: None,
            device_ids,
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
                Ok(weather_record) => {
                    let weather_reading = Station::report_r1_to_reading(&weather_record);

                    // keep track of the last recorded temp. Used for other calculations
                    self.last_recorded_temp = weather_reading.out_temp;

                    // write the result
                    let write_result = self.writer.write(&weather_reading).await;

                    if write_result.is_ok() {
                        println!("{:?}", weather_reading);
                    }

                    set_timeout(Duration::from_secs(18)).await;
                }
                Err(_) => {
                    println!("There was a problem reading report R1.");

                    set_timeout(Duration::from_secs(30)).await; // wait for a bit

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

        while !is_open {
            let open_result = self.hid.open(self.device_ids.vid, self.device_ids.pid);

            match open_result {
                Ok(device) => {
                    println!("HID device open...",);

                    is_open = true;

                    self.device = Some(device);
                }
                Err(_) => {
                    if retry_attempts > max_retry_attempts {
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

                        set_timeout(Duration::from_secs(10)).await;
                    }
                }
            }
        }
    }

    /**
     * Read and decode report R1
     */
    fn read_report_r1(&self) -> Result<WeatherRecord, &str> {
        if let Some(d) = &self.device {
            let mut buf: Report1 = [1u8; 10];

            let res = d.get_feature_report(&mut buf);

            match res {
                Ok(_) => Ok(Station::decode_r1(&buf, self.last_recorded_temp)),
                Err(_) => Err("Failed to read report"),
            }
        } else {
            Err("Failed to read report")
        }
    }

    fn decode_r1(data: &Report1, prev_temp: Option<f32>) -> WeatherRecord {
        let report_flavor = Station::decode_r1_flavor(&data);

        if report_flavor == 1 {
            WeatherRecord::Type1(Station::decode_r1_t1(data, prev_temp))
        } else {
            WeatherRecord::Type2(Station::decode_r1_t2(data))
        }
    }

    fn report_r1_to_reading(weather_record: &WeatherRecord) -> WeatherReading {
        let time = Timestamp::from(Utc::now()).into();

        match weather_record {
            WeatherRecord::Type1(value) => WeatherReading {
                time,
                rain: Some(value.rain),
                wind_speed: Some(value.wind_speed),
                out_temp: value.out_temp,
                out_humid: None,
                wind_chill: value.wind_chill,
            },

            WeatherRecord::Type2(value) => WeatherReading {
                time,
                rain: None,
                wind_speed: Some(value.wind_speed),
                out_temp: Some(value.out_temp),
                out_humid: Some(value.out_humid),
                wind_chill: Some(value.wind_chill),
            },
        }
    }

    fn decode_r1_t1(data: &Report1, prev_temp: Option<f32>) -> WeatherRecordType1 {
        let rain = Station::decode_rain(data);
        let wind_speed = Station::decode_wind_speed(data);

        if let Some(temp) = prev_temp {
            // If we have a previous temp calculate new wind chill
            let wind_chill = calc_wind_chill(wind_speed, temp);

            WeatherRecordType1 {
                wind_speed,
                wind_chill: Some(wind_chill),
                out_temp: Some(temp),
                rain,
            }
        } else {
            WeatherRecordType1 {
                wind_speed,
                wind_chill: None,
                out_temp: None,
                rain,
            }
        }
    }

    fn decode_r1_t2(data: &Report1) -> WeatherRecordType2 {
        let wind_speed = Station::decode_wind_speed(data);
        let out_temp = Station::decode_out_temp(&data);

        WeatherRecordType2 {
            wind_speed,
            out_temp,
            out_humid: Station::decode_humidity(&data),
            wind_chill: calc_wind_chill(wind_speed, out_temp),
        }
    }

    fn decode_wind_speed(data: &Report1) -> f32 {
        let n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

        if n == 0 {
            return 0.0;
        }

        ((0.8278 * n as f32 + 1.0) / 1.609).round()
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

    fn decode_humidity(data: &Report1) -> u8 {
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
    fn decode_r1_type_1() {
        let raw: [u8; 10] = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let r = Station::decode_r1(&raw, Some(31.499998));

        if let WeatherRecord::Type1(val) = r {
            assert_eq!(val.rain, 1.08);
            assert_eq!(val.wind_speed, 3.0);
            assert_eq!(val.wind_chill, Some(28.75116));
        } else {
            panic!("record is not of type 1");
        }
    }

    #[test]
    fn decode_r1_type_2() {
        let raw: [u8; 10] = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let r = Station::decode_r1(&raw, Some(31.499998));

        if let WeatherRecord::Type2(val) = r {
            assert_eq!(val.wind_speed, 0.0);
            assert_eq!(val.out_temp, 31.499998);
            assert_eq!(val.out_humid, 75);
            assert_eq!(val.wind_chill, 31.499998);
        } else {
            panic!("record is not of type 2");
        }
    }
}
