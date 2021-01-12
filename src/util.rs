pub fn calc_wind_chill(wind_speed: f32, out_temp: f32) -> f32 {
    if wind_speed < 3. {
        return out_temp;
    }

    let speed = wind_speed.powf(0.16);
    let raw = 35.74 + 0.6215 * out_temp - 35.75 * speed + 0.4275 * out_temp * speed;

    raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_wind_chill_above_2() {
        assert_eq!(calc_wind_chill(3., 38.), 36.10366);
    }

    #[test]
    fn test_calc_wind_chill_below_3() {
        assert_eq!(calc_wind_chill(3., 38.), 36.10366);
    }
}
