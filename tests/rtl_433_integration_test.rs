mod test_reader;
mod test_writer;

#[tokio::test]
async fn shold_read_and_record_readings() {
    let mut reader = test_reader::RTL433TestReader {
        current_reading: 0,
        readings: vec![
            String::from("{\"time\" : \"2021-12-15T20:48:18Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 3.193, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:48:37Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 49, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 2.679, \"wind_dir_deg\" : 90.000, \"rain_in\" : 41.830, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:48:56Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 4.222, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:49:14Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 49, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 4.736, \"wind_dir_deg\" : 180.000, \"rain_in\" : 42.830, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:49:33Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 2.679, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")
        ],
    };

    let mut writer = test_writer::TestWriter { readings: vec![] };
    let mut station = acurite_rtl_433::Station::new();

    // generate some readings
    for _ in 1..=5 {
        station.run(&mut reader, &mut writer).await;
    }

    // Get stored readings from the writer
    let data = writer.readings.into_iter();

    // Check writers stored weahter properties
    let rain: Vec<Option<f32>> = data.clone().map(|r| r.rain).collect();
    let rain_delta: Vec<Option<f32>> = data.clone().map(|r| r.rain_delta).collect();
    let wind_speed: Vec<Option<f32>> = data.clone().map(|r| r.wind_speed).collect();
    let wind_dir: Vec<Option<f32>> = data.clone().map(|r| r.wind_dir).collect();
    let out_temp: Vec<Option<f32>> = data.clone().map(|r| r.out_temp).collect();
    let out_humid: Vec<Option<u8>> = data.clone().map(|r| r.out_humid).collect();
    let wind_chill: Vec<Option<f32>> = data.clone().map(|r| r.wind_chill).collect();
    let heat_index: Vec<Option<f32>> = data.clone().map(|r| r.heat_index).collect();
    let dew_point: Vec<Option<f32>> = data.clone().map(|r| r.dew_point).collect();

    assert_eq!(
        rain,
        [None, Some(41.83), Some(41.83), Some(42.83), Some(42.83)]
    );
    assert_eq!(
        rain_delta,
        [Some(0.0), Some(0.0), Some(0.0), Some(1.0), Some(0.0)]
    );
    assert_eq!(
        wind_speed,
        [
            Some(3.193),
            Some(2.679),
            Some(4.222),
            Some(4.736),
            Some(2.679)
        ]
    );
    assert_eq!(
        wind_dir,
        [None, Some(90.0), Some(90.0), Some(180.0), Some(180.0)]
    );
    assert_eq!(
        out_temp,
        [Some(55.8), Some(55.8), Some(55.8), Some(55.8), Some(55.8)]
    );
    assert_eq!(
        out_humid,
        [Some(70), Some(70), Some(70), Some(70), Some(70)]
    );
    assert_eq!(
        wind_chill,
        [Some(55.8), Some(55.8), Some(55.8), Some(55.8), Some(55.8)]
    );
    assert_eq!(
        heat_index,
        [
            Some(54.370003),
            Some(54.370003),
            Some(54.370003),
            Some(54.370003),
            Some(54.370003)
        ]
    );
    assert_eq!(
        dew_point,
        [
            Some(46.282158),
            Some(46.282158),
            Some(46.282158),
            Some(46.282158),
            Some(46.282158)
        ]
    );
}
