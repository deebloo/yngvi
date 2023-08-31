use ws_core::InMemWriter;
use ws_rtl_433::{rtl_433_file_source, RTL433Reader};

mod error_writer;

#[tokio::test]
async fn should_replay_failed_writes_rtl_433() {
    let mut station = ws_core::Station::new();

    let source = rtl_433_file_source("tests/data/rtl_433_data_1.txt");
    let reader = RTL433Reader::new(source);

    let mut writer = error_writer::ErrorWriter {};

    station.start(reader, &mut writer).await;

    assert_eq!(station.retry_manager.failed_writes.len(), 5);

    let reader = RTL433Reader::new(vec![
        Ok(String::from("{\"time\" : \"2021-12-15T20:48:18Z\", \"model\" : \"Acurite-5n1\", \"message_type\" : 56, \"id\" : 1306, \"channel\" : \"A\", \"sequence_num\" : 0, \"battery_ok\" : 1, \"wind_avg_mi_h\" : 3.193, \"temperature_F\" : 55.800, \"humidity\" : 70, \"mic\" : \"CHECKSUM\"}")),
    ].into_iter());

    let mut writer = InMemWriter::new();

    station.start(reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 6);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}
