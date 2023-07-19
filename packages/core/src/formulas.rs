// Calculated based on formula from the National Weather Service
// https://www.weather.gov/media/epz/wxcalc/windChill.pdf
pub fn calc_wind_chill(wind_speed: f32, out_temp: f32) -> f32 {
    if wind_speed < 3. || out_temp >= 50. {
        return out_temp;
    }

    let speed = wind_speed.powf(0.16);
    let raw = 35.74 + 0.6215 * out_temp - 35.75 * speed + 0.4275 * out_temp * speed;

    raw
}

// Calculated based on formula from the National Weather Service
// https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml
pub fn calc_heat_index(temp: f32, humid: u8) -> f32 {
    if temp <= 40. {
        return temp;
    }

    let rh = humid as f32;

    let mut hi = 0.5 * (temp + 61.0 + ((temp - 68.0) * 1.2) + (rh * 0.094));

    if hi > 79.0 {
        hi = -42.379 + 2.04901523 * temp + 10.14333127 * rh
            - 0.22475541 * temp * rh
            - 0.00683783 * temp * temp
            - 0.05481717 * rh * rh
            + 0.00122874 * temp * temp * rh
            + 0.00085282 * temp * rh * rh
            - 0.00000199 * temp * temp * rh * rh;

        if rh <= 13. && temp >= 80. && temp <= 112. {
            hi = hi - ((13. - rh) / 4.) * ((17. - (temp - 95.).abs()) / 17.).sqrt();
        } else if rh > 85. && temp >= 80. && temp <= 87. {
            hi = hi - ((rh - 85.) / 10.) * ((87. - temp) / 5.);
        }
    }

    hi
}

// Based on simple Dew Point calculation
// https://iridl.ldeo.columbia.edu/dochelp/QA/Basic/dewpoint.html
pub fn calc_dew_point(temp: f32, humid: u8) -> f32 {
    let temp_c = (temp - 32.) / 1.8;
    let rh = humid as f32;

    let res = temp_c
        - (14.55 + 0.114 * temp_c) * (1. - (0.01 * rh))
        - ((2.5 + 0.007 * temp_c) * (1. - (0.01 * rh))).powf(3.)
        - (15.9 + 0.117 * temp_c) * (1. - (0.01 * rh)).powf(14.);

    res * (9.0 / 5.0) + 32.
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
        assert_eq!(calc_wind_chill(3., 38.), 36.10366);
    }

    #[test]
    fn test_calc_wind_chill_below_3() {
        assert_eq!(calc_wind_chill(2., 38.), 38.);
    }

    #[test]
    fn test_calc_wind_chill_temp_above_50() {
        assert_eq!(calc_wind_chill(2., 51.), 51.);
    }

    #[test]
    fn test_calc_hi_should_be_temp() {
        assert_eq!(calc_heat_index(26., 50), 26.);
    }

    #[test]
    fn test_calc_hi_should_be_correct_simple() {
        assert_eq!(calc_heat_index(75., 100), 76.899994);
    }

    #[test]
    fn test_calc_hi_should_be_correct_no_adjustment() {
        assert_eq!(calc_heat_index(80., 65), 82.36536);
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_1() {
        assert_eq!(calc_heat_index(85., 10), 81.39988);
    }

    #[test]
    fn test_calc_hi_should_be_correct_adjustment_2() {
        assert_eq!(calc_heat_index(85., 90), 101.38081);
    }

    #[test]
    fn calc_dewpoint_should_work() {
        assert_eq!(calc_dew_point(79., 50).round(), 59.0);
    }

    #[test]
    fn map_correct_() {
        // 271..=359 => "NW",
        // 181..=269 => "SW",
        // 91..=179 => "SE",
        // 1..=89 => "NE",
        // 0 => "N",
        // 90 => "E",
        // 180 => "S",
        // 270 => "W",
        // _ => "",

        assert_eq!(wind_dir_to_cardinal(0), "N");
        assert_eq!(wind_dir_to_cardinal(90), "E");
        assert_eq!(wind_dir_to_cardinal(180), "S");
        assert_eq!(wind_dir_to_cardinal(270), "W");
        assert_eq!(wind_dir_to_cardinal(300), "NW");
        assert_eq!(wind_dir_to_cardinal(200), "SW");
        assert_eq!(wind_dir_to_cardinal(150), "NW");
        assert_eq!(wind_dir_to_cardinal(50), "NW");
    }
}
