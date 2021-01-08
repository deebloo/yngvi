use crate::util::calc_wind_chill;
use crate::writer::Writer;

use hidapi::HidDevice;
use settimeout::set_timeout;
use std::time::Duration;

type Report1 = [u8; 10];

#[derive(Debug)]
pub struct WeatherRecordType1 {
    pub wind_speed: f32,
    pub rain: f32,
}

#[derive(Debug)]
pub struct WeatherRecordType2 {
    pub wind_speed: f32,
    pub out_temp: f32,
    pub out_humid: u8,
    pub wind_chill: f32,
}

#[derive(Debug)]
pub enum WeatherRecord {
    Type1(WeatherRecordType1),
    Type2(WeatherRecordType2),
}

pub struct Station<'a> {
    device: &'a HidDevice,
}
impl<'a> Station<'a> {
    pub fn new(device: &'a HidDevice) -> Station {
        Station { device }
    }

    pub async fn start(&self, writer: &Writer<'a>) {
        loop {
            let report = self.read_report_r1();
            let write_result = writer.write(&report).await;

            assert!(write_result.is_ok(), "Write result was not okay");
            println!("{:?}", report);

            set_timeout(Duration::from_secs(18)).await;
        }
    }

    pub fn read_report_r1(&self) -> WeatherRecord {
        // Read data from device
        let mut buf: Report1 = [1u8; 10];

        self.device.get_feature_report(&mut buf).unwrap();

        Station::decode_r1(&buf)
    }

    pub fn decode_r1(data: &Report1) -> WeatherRecord {
        let wind_speed = Station::decode_wind_speed(data);

        if Station::decode_r1_flavor(&data) == 1 {
            return WeatherRecord::Type1(WeatherRecordType1 {
                wind_speed,
                rain: Station::decode_rain(&data),
            });
        }

        let out_temp = Station::decode_out_temp(&data);

        WeatherRecord::Type2(WeatherRecordType2 {
            wind_speed,
            out_temp,
            out_humid: Station::decode_humidity(&data),
            wind_chill: calc_wind_chill(wind_speed, out_temp),
        })
    }

    pub fn decode_wind_speed(data: &Report1) -> f32 {
        let n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

        if n == 0 {
            return 0.0;
        }

        ((0.8278 * n as f32 + 1.0) / 1.609).round()
    }

    pub fn decode_r1_flavor(data: &Report1) -> u8 {
        data[3] & 0x0f
    }

    pub fn decode_out_temp(data: &Report1) -> f32 {
        let a = ((data[5] & 0x0f) as u32) << 7;
        let b = (data[6] & 0x7f) as u32;
        let celcius = (a | b) as f32 / 18.0 - 40.0;

        (celcius * 9.) / 5. + 32.
    }

    pub fn decode_humidity(data: &Report1) -> u8 {
        data[7]
    }

    pub fn decode_rain(data: &Report1) -> f32 {
        let cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) as f32 * 0.0254;

        cm / 2.54
    }
}
