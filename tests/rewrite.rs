use rtl_433::RTL433Reader;
use weather::InMemWriter;

mod error_writer;

#[tokio::test]
async fn should_replay_failed_writes_rtl_433() {
    let mut station = weather::Station::new();

    let reader = RTL433Reader::new(vec![
        Ok(String::from("{\"time\" : \"2021-12-15T20:48:18Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 0, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 3.193, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")),
        Ok(String::from("{\"time\" : \"2021-12-15T20:48:37Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 49, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 0, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 2.679, \"wind_dir_deg\" : 90.000, \"rain_in\" : 41.830, \"mic\" : \"CHECKSUM\"}")),
        Ok(String::from("{\"time\" : \"2021-12-15T20:48:56Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 0, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 4.222, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")),
    ].into_iter());

    let mut writer = error_writer::ErrorWriter {};

    station.start(reader, &mut writer).await;

    assert_eq!(station.retry_manager.failed_writes.len(), 3);

    let reader = RTL433Reader::new(vec![
        Ok(String::from("{\"time\" : \"2021-12-15T20:48:18Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 0, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 3.193, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")),
    ].into_iter());

    let mut writer = InMemWriter { readings: vec![] };

    station.start(reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 4);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}
