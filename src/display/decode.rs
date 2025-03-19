use metrum::Temp;

pub type Report1 = [u8; 10];

pub const WIND_DIR_BY_IDX: [f64; 16] = [
    315.0, 247.5, 292.5, 270.0, 337.5, 225.0, 0.0, 202.5, 67.5, 135.0, 90.0, 112.5, 45.0, 157.5,
    22.5, 180.0,
];

pub fn decode_flavor(data: &Report1) -> u8 {
    data[3] & 0x0f
}

pub fn decode_wind_speed(data: &Report1) -> f64 {
    let n = ((data[4] & 0x1f) << 3) | ((data[5] & 0x70) >> 4);

    if n == 0 {
        return 0.0;
    }

    (0.8278 * n as f64 + 1.0) / 1.609
}

pub fn decode_out_temp(data: &Report1) -> Temp {
    let a = ((data[5] & 0x0f) as u32) << 7;
    let b = (data[6] & 0x7f) as u32;
    let celcius = (a | b) as f64 / 18.0 - 40.0;

    Temp::from_c(celcius)
}

pub fn decode_out_humidity(data: &Report1) -> u8 {
    data[7] & 0x7f
}

pub fn decode_rain(data: &Report1) -> f64 {
    let cm = (((data[6] & 0x3f) << 7) | (data[7] & 0x7f)) as f64 * 0.0254;

    cm / 2.54
}

pub fn decode_wind_dir(data: &Report1) -> f64 {
    let index = data[5] & 0x0f;

    WIND_DIR_BY_IDX[index as usize]
}

pub fn validate_r1(data: &Report1) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_r1_falvor_1() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let flavor = decode_flavor(&report);

        assert_eq!(flavor, 1);
    }

    #[test]
    fn decode_r1_falvor_8() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let flavor = decode_flavor(&report);

        assert_eq!(flavor, 8);
    }

    #[test]
    fn decode_rain_check() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let rain = decode_rain(&report);

        assert_eq!(rain, 1.0799999999999998);
    }

    #[test]
    fn decode_wind_speed_check() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let wind_speed = decode_wind_speed(&report);

        assert_eq!(wind_speed, 2.6794283);
    }

    #[test]
    fn decode_wind_dir_check() {
        let report: Report1 = [1, 197, 26, 113, 0, 200, 0, 108, 3, 255];
        let wind_dir = decode_wind_dir(&report);

        assert_eq!(wind_dir, 67.5);
    }

    #[test]
    fn decode_out_temp_check() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let out_temp = decode_out_temp(&report);

        assert_eq!(out_temp, Temp::from_f(31.5));
    }

    #[test]
    fn decode_out_humid_check() {
        let report: Report1 = [1, 197, 26, 120, 0, 5, 75, 75, 3, 255];
        let out_humid = decode_out_humidity(&report);

        assert_eq!(out_humid, 75);
    }
}
