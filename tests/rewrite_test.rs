mod test_reader;
mod test_writer;

#[tokio::test]
async fn should_replay_failed_writes_console() {
    let mut reader = test_reader::TestReader {
        current_reading: 0,
        readings: vec![
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
        ],
    };

    let mut writer = test_writer::TestWriter { readings: vec![] };
    let mut error_writer = test_writer::ErrorWriter {};
    let mut station = acurite_console::Station::new();

    // Error Out 3 times
    for _ in 1..=3 {
        station.run(&mut reader, &mut error_writer).await;
    }

    assert_eq!(station.retry_manager.failed_writes.len(), 3);

    station.run(&mut reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 4);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}

#[tokio::test]
async fn should_replay_failed_writes_rtl_433() {
    let mut reader = test_reader::RTL433TestReader {
        current_reading: 0,
        readings: vec![
            String::from("{\"time\" : \"2021-12-15T20:48:18Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 3.193, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:48:37Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 49, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 2.679, \"wind_dir_deg\" : 90.000, \"rain_in\" : 41.830, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:48:56Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 4.222, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:49:14Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 49, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 4.736, \"wind_dir_deg\" : 180.000, \"rain_in\" : 41.830, \"mic\" : \"CHECKSUM\"}"),
            String::from("{\"time\" : \"2021-12-15T20:49:33Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 2, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 2.679, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")
        ],
    };

    let mut writer = test_writer::TestWriter { readings: vec![] };
    let mut error_writer = test_writer::ErrorWriter {};
    let mut station = acurite_rtl_433::Station::new();

    // Error Out 3 times
    for _ in 1..=3 {
        station.run(&mut reader, &mut error_writer).await;
    }

    assert_eq!(station.retry_manager.failed_writes.len(), 3);

    station.run(&mut reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 4);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}
