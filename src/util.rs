pub fn calc_wind_chill(wind_speed: f32, out_temp: f32) -> f32 {
    if wind_speed < 3. {
        return out_temp as f32;
    }

    let raw = 35.74 + 0.6215 * out_temp as f32 - 35.75 * wind_speed.powf(0.16)
        + 0.4275 * out_temp as f32 * wind_speed.powf(0.16);

    raw
}
