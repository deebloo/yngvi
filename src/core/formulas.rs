use metrum::Temp;

// Calculated based on formula from the National Weather Service
// https://www.weather.gov/media/epz/wxcalc/windChill.pdf
pub fn calc_wind_chill(wind_speed: f64, out_temp: Temp) -> Temp {
    // formula only works in F
    let start_temp = out_temp.as_f();

    if wind_speed < 3. || start_temp >= 50. {
        return out_temp;
    }

    let speed = wind_speed.powf(0.16);
    let raw = 35.74 + 0.6215 * start_temp - 35.75 * speed + 0.4275 * start_temp * speed;

    Temp::from_f((raw * 100.0).round() / 100.0)
}

// Calculated based on formula from the National Weather Service
// https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml
pub fn calc_heat_index(out_temp: Temp, humid: u8) -> Temp {
    // formula only works in F
    let start_temp = out_temp.as_f();

    if start_temp <= 40. {
        return out_temp;
    }

    let rh = humid as f64;

    let mut hi = 0.5 * (start_temp + 61.0 + ((start_temp - 68.0) * 1.2) + (rh * 0.094));

    if hi > 79.0 {
        hi = -42.379 + 2.04901523 * start_temp + 10.14333127 * rh
            - 0.22475541 * start_temp * rh
            - 0.00683783 * start_temp * start_temp
            - 0.05481717 * rh * rh
            + 0.00122874 * start_temp * start_temp * rh
            + 0.00085282 * start_temp * rh * rh
            - 0.00000199 * start_temp * start_temp * rh * rh;

        if rh <= 13. && start_temp >= 80. && start_temp <= 112. {
            hi = hi - ((13. - rh) / 4.) * ((17. - (start_temp - 95.).abs()) / 17.).sqrt();
        } else if rh > 85. && start_temp >= 80. && start_temp <= 87. {
            hi = hi - ((rh - 85.) / 10.) * ((87. - start_temp) / 5.);
        }
    }

    Temp::from_f((hi * 100.0).round() / 100.0)
}

// Based on simple Dew Point calculation
// https://iridl.ldeo.columbia.edu/dochelp/QA/Basic/dewpoint.html
pub fn calc_dew_point(temp: Temp, humid: u8) -> Temp {
    let temp_c = temp.as_c();

    let rh = humid as f64;

    let res = temp_c
        - (14.55 + 0.114 * temp_c) * (1. - (0.01 * rh))
        - ((2.5 + 0.007 * temp_c) * (1. - (0.01 * rh))).powf(3.)
        - (15.9 + 0.117 * temp_c) * (1. - (0.01 * rh)).powf(14.);

    Temp::from_c(res)
}

pub fn wind_dir_to_cardinal<'a>(wind_dir: f64) -> &'a str {
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
        assert_eq!(calc_wind_chill(3., Temp::from_f(38.)), Temp::from_f(36.1));
    }

    #[test]
    fn test_calc_wind_chill_below_3() {
        assert_eq!(calc_wind_chill(2., Temp::from_f(38.)), Temp::from_f(38.));
    }

    #[test]
    fn test_calc_wind_chill_temp_above_50() {
        assert_eq!(calc_wind_chill(2., Temp::from_f(51.)), Temp::from_f(51.));
    }

    #[test]
    fn test_calc_hi_should_be_temp() {
        assert_eq!(calc_heat_index(Temp::from_f(26.), 50), Temp::from_f(26.));
    }

    #[test]
    fn test_calc_hi_should_be_correct_simple() {
        assert_eq!(calc_heat_index(Temp::from_f(75.), 100), Temp::from_f(76.9));
    }

    #[test]
    fn test_calc_hi_should_be_correct_no_adjustment() {
        assert_eq!(calc_heat_index(Temp::from_f(80.), 65), Temp::from_f(82.37));
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_1() {
        assert_eq!(calc_heat_index(Temp::from_f(85.), 10), Temp::from_f(81.4));
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_2() {
        assert_eq!(
            calc_heat_index(Temp::from_f(85.), 90),
            Temp::from_f(101.37999999999993861)
        );
    }

    #[test]
    fn calc_dewpoint_should_work() {
        assert_eq!(
            calc_dew_point(Temp::from_f(79.), 50),
            Temp::from_f(58.879449378347963773)
        );
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
