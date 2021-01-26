use async_std::task;
use chrono::Utc;
use hidapi::{HidApi, HidDevice};
use std::time::Duration;

use crate::util::calc_wind_chill;
use crate::writer::{WeatherReading, Writer};

type Report1 = [u8; 10];

pub struct DeviceIds {
    pub vid: u16,
    pub pid: u16,
}

enum StationError {
    R1ReadError,
    NoDevice,
    R1ReportInvalid(Report1),
}

const WIND_DIR_BY_IDX: [f32; 16] = [
    315.0, 247.5, 292.5, 270.0, 337.5, 225.0, 0.0, 202.5, 67.5, 135.0, 90.0, 112.5, 45.0, 157.5,
    22.5, 180.0,
];

pub struct Station<'a> {
    pub hid: &'a HidApi,
    pub writer: &'a dyn Writer,
    pub device_ids: DeviceIds,
    weather_reading: WeatherReading,
    device: Option<HidDevice>,
    is_running: bool,
}

impl<'a> Station<'a> {
    pub fn new(hid: &'a HidApi, device_ids: DeviceIds, writer: &'a impl Writer) -> Station<'a> {
        Station {
            hid,
            writer,
            device_ids,
            weather_reading: WeatherReading::new(),
            device: None,
            is_running: false,
        }
    }

    /**
     * Open device and start reading reports.
     * If a failure to read occurs wait and then re-open device
     */
    pub async fn start(&mut self) {
        self.is_running = true;
        self.open_device().await;

        while self.is_running {
            match self.read_report_r1() {
                Ok(report) => {
                    self.weather_reading.time = Utc::now();

                    self.update_weather_reading_r1(report);

                    let write_result = self.writer.write(&self.weather_reading).await;

                    if write_result.is_ok() {
                        println!("{:?}", self.weather_reading);
                    }
                }

                Err(StationError::R1ReadError) => {
                    println!("Failed to read report R1");

                    self.open_device().await;
                }

                Err(StationError::R1ReportInvalid(report)) => {
                    println!("Report R1 Invalid {:?}", report);
                }

                Err(StationError::NoDevice) => {
                    println!("No device found");

                    self.open_device().await;
                }
            }
            task::sleep(Duration::from_secs(18)).await;
        }
    }

    /**
     * Open HID device.
     * Attempt to connect 5 times
     */
    async fn open_device(&mut self) {
        println!("Opening HID device...");

        self.device = None;

        let open_result = self.hid.open(self.device_ids.vid, self.device_ids.pid);

        if let Ok(device) = open_result {
            self.device = Some(device);
        }
    }

    /**
     * Read and decode report R1
     */
    fn read_report_r1(&self) -> Result<Report1, StationError> {
        if let Some(d) = &self.device {
            let mut buf: Report1 = [1u8; 10];
            let res = d.get_feature_report(&mut buf);

            match res {
                Ok(_) => {
                    let is_valid = Station::validate_r1(&buf);

                    if is_valid {
                        Ok(buf)
                    } else {
                        Err(StationError::R1ReportInvalid(buf))
                    }
                }
                Err(err) => {
                    println!("{:?}", err);

                    Err(StationError::R1ReadError)
                }
            }
        } else {
            Err(StationError::NoDevice)
        }
    }

    fn update_weather_reading_r1(&mut self, data: Report1) {
        let report_flavor = Station::decode_flavor(&data);

        // Both flavors have wind speed
        let wind_speed = Station::decode_wind_speed(&data);
        self.weather_reading.wind_speed = Some(wind_speed);

        if report_flavor == 1 {
            let new_rain_total = Station::decode_rain(&data);

            if let Some(prev_rain_total) = self.weather_reading.rain {
                if new_rain_total >= prev_rain_total {
                    self.weather_reading.rain_delta = Some(new_rain_total - prev_rain_total);
                }
            }

            self.weather_reading.rain = Some(new_rain_total);
            self.weather_reading.wind_dir = Some(Station::decode_wind_dir(&data));

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

    fn decode_flavor(data: &Report1) -> u8 {
        data[3] & 0x0f
    }

    fn decode_wind_speed(data: &Report1) -> f32 {
        let n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

        if n == 0 {
            return 0.0;
        }

        (0.8278 * n as f32 + 1.0) / 1.609
    }

    fn decode_out_temp(data: &Report1) -> f32 {
        let a = ((data[5] & 0x0f) as u32) << 7;
        let b = (data[6] & 0x7f) as u32;
        let celcius = (a | b) as f32 / 18.0 - 40.0;

        (celcius * 9.) / 5. + 32.
    }

    fn decode_out_humidity(data: &Report1) -> u8 {
        data[7] & 0x7f
    }

    fn decode_rain(data: &Report1) -> f32 {
        let cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) as f32 * 0.0254;

        cm / 2.54
    }

    fn decode_wind_dir(data: &Report1) -> f32 {
        let index = data[5] & 0x0f;

        WIND_DIR_BY_IDX[index as usize]
    }

    fn validate_r1(data: &Report1) -> bool {
        if data[1] & 0x0f == 0x0f && data[3] == 0xff {
            println!("R1: no sensors found");

            false
        } else if data[3] & 0x0f != 1 && data[3] & 0x0f != 8 {
            println!("R1: invalid message flavor");

            false
        } else if data[9] != 0xff && data[9] != 0x00 {
            println!("R1: invalid final byte");

            false
        } else if data[8] & 0x0f > 3 {
            println!("R1: invalid signal strength");

            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use lazy_static::lazy_static;

    struct MockWriter;
    #[async_trait]
    impl Writer for MockWriter {
        async fn write(&self, _weather_reading: &WeatherReading) -> Result<(), ()> {
            Ok({})
        }
    }

    lazy_static! {
        static ref HID: HidApi = HidApi::new().unwrap();
    }

    #[test]
    fn decode_r1_falvor_1() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let flavor = Station::decode_flavor(&report);

        assert_eq!(flavor, 1);
    }

    #[test]
    fn decode_r1_falvor_8() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let flavor = Station::decode_flavor(&report);

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
    fn decode_wind_dir() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let wind_dir = Station::decode_wind_dir(&report);

        assert_eq!(wind_dir, 67.5);
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

    #[test]
    fn creates_correct_reading() {
        let writer = MockWriter {};

        let mut station = Station::new(&HID, DeviceIds { vid: 0, pid: 1 }, &writer);

        station.update_weather_reading_r1([1, 197, 26, 120, 0, 5, 75, 75, 3, 255]);

        assert_eq!(
            station.weather_reading,
            WeatherReading {
                time: station.weather_reading.time,
                rain: None,
                rain_delta: None,
                wind_speed: Some(0.0),
                wind_dir: None,
                out_temp: Some(31.499998),
                out_humid: Some(75),
                wind_chill: Some(31.499998)
            }
        )
    }

    #[test]
    fn calculates_rain_delta() {
        let writer = MockWriter {};

        let mut station = Station::new(&HID, DeviceIds { vid: 0, pid: 1 }, &writer);

        // rain total = 1.08
        station.update_weather_reading_r1([1, 197, 26, 113, 0, 200, 0, 108, 3, 255]);

        // rain total = 2.3600001
        station.update_weather_reading_r1([1, 197, 26, 113, 0, 200, 1, 108, 3, 255]);

        println!("{:?}", station.weather_reading);

        assert_eq!(station.weather_reading.rain_delta, Some(1.2800001))
    }
}
