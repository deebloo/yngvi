use degrees::Temp;

// Calculated based on formula from the National Weather Service
// https://www.weather.gov/media/epz/wxcalc/windChill.pdf
pub fn calc_wind_chill(wind_speed: f32, out_temp: Temp) -> Temp {
    // formula only works in F
    let start_temp = out_temp.as_f();

    if wind_speed < 3. || start_temp >= Temp::F(50.) {
        return out_temp;
    }

    let raw_temp: f32 = out_temp.into();
    let speed = wind_speed.powf(0.16);
    let raw = 35.74 + 0.6215 * raw_temp - 35.75 * speed + 0.4275 * raw_temp * speed;

    Temp::F(raw).round()
}

// Calculated based on formula from the National Weather Service
// https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml
pub fn calc_heat_index(out_temp: Temp, humid: u8) -> Temp {
    // formula only works in F
    let start_temp = out_temp.as_f();

    if start_temp <= Temp::F(40.) {
        return start_temp;
    }

    let rh = humid as f32;
    let raw_temp: f32 = start_temp.into();

    let mut hi = 0.5 * (raw_temp + 61.0 + ((raw_temp - 68.0) * 1.2) + (rh * 0.094));

    if hi > 79.0 {
        hi = -42.379 + 2.04901523 * raw_temp + 10.14333127 * rh
            - 0.22475541 * raw_temp * rh
            - 0.00683783 * raw_temp * raw_temp
            - 0.05481717 * rh * rh
            + 0.00122874 * raw_temp * raw_temp * rh
            + 0.00085282 * raw_temp * rh * rh
            - 0.00000199 * raw_temp * raw_temp * rh * rh;

        if rh <= 13. && start_temp >= Temp::F(80.) && start_temp <= Temp::F(112.) {
            hi = hi - ((13. - rh) / 4.) * ((17. - (raw_temp - 95.).abs()) / 17.).sqrt();
        } else if rh > 85. && start_temp >= Temp::F(80.) && start_temp <= Temp::F(87.) {
            hi = hi - ((rh - 85.) / 10.) * ((87. - raw_temp) / 5.);
        }
    }

    Temp::F(hi).round()
}

// Based on simple Dew Point calculation
// https://iridl.ldeo.columbia.edu/dochelp/QA/Basic/dewpoint.html
pub fn calc_dew_point(temp: Temp, humid: u8) -> Temp {
    let temp_c: f32 = temp.as_c().into();

    let rh = humid as f32;

    let res = temp_c
        - (14.55 + 0.114 * temp_c) * (1. - (0.01 * rh))
        - ((2.5 + 0.007 * temp_c) * (1. - (0.01 * rh))).powf(3.)
        - (15.9 + 0.117 * temp_c) * (1. - (0.01 * rh)).powf(14.);

    Temp::C(res).as_f()
}

pub fn wind_dir_to_cardinal<'a>(wind_dir: f32) -> &'a str {
    match wind_dir as u32 {
        271..=359 => "NW",
        181..=269 => "SW",
        91..=179 => "SE",
        1..=89 => "NE",
        0 => "N",
        90 => "E",
        180 => "S",
        270 => "W",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_wind_chill_at_least_3() {
        println!("{:?}", calc_wind_chill(3., Temp::F(38.)));

        assert_eq!(calc_wind_chill(3., Temp::F(38.)), Temp::F(36.104));
    }

    #[test]
    fn test_calc_wind_chill_below_3() {
        assert_eq!(calc_wind_chill(2., Temp::F(38.)), Temp::F(38.));
    }

    #[test]
    fn test_calc_wind_chill_temp_above_50() {
        assert_eq!(calc_wind_chill(2., Temp::F(51.)), Temp::F(51.));
    }

    #[test]
    fn test_calc_hi_should_be_temp() {
        assert_eq!(calc_heat_index(Temp::F(26.), 50), Temp::F(26.));
    }

    #[test]
    fn test_calc_hi_should_be_correct_simple() {
        assert_eq!(calc_heat_index(Temp::F(75.), 100), Temp::F(76.9));
    }

    #[test]
    fn test_calc_hi_should_be_correct_no_adjustment() {
        assert_eq!(calc_heat_index(Temp::F(80.), 65), Temp::F(82.365));
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_1() {
        assert_eq!(calc_heat_index(Temp::F(85.), 10), Temp::F(81.4));
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_2() {
        assert_eq!(calc_heat_index(Temp::F(85.), 90), Temp::F(101.381));
    }

    #[test]
    fn calc_dewpoint_should_work() {
        assert_eq!(calc_dew_point(Temp::F(79.), 50), Temp::F(58.88));
    }

    #[test]
    fn map_correct_deg_to_direction() {
        assert_eq!(wind_dir_to_cardinal(0.0), "N");
        assert_eq!(wind_dir_to_cardinal(90.0), "E");
        assert_eq!(wind_dir_to_cardinal(180.0), "S");
        assert_eq!(wind_dir_to_cardinal(270.0), "W");
        assert_eq!(wind_dir_to_cardinal(300.0), "NW");
        assert_eq!(wind_dir_to_cardinal(200.0), "SW");
        assert_eq!(wind_dir_to_cardinal(150.0), "SE");
        assert_eq!(wind_dir_to_cardinal(50.0), "NE");
    }
}
