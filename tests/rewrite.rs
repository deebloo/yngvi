use ws_core::InMemWriter;
use ws_rtl_433::{rtl_433_file_source, RTL433Reader};

mod error_writer;

#[tokio::test]
async fn should_replay_failed_writes_rtl_433() {
    let mut station = ws_core::Station::new();

    let source = rtl_433_file_source("data/rtl_433.txt").take(5);
    let reader = RTL433Reader::new(source);

    let mut writer = error_writer::ErrorWriter {};

    station.start(reader, &mut writer).await;

    assert_eq!(station.retry_manager.failed_writes.len(), 5);

    let source = rtl_433_file_source("data/rtl_433.txt").take(5);
    let reader = RTL433Reader::new(source.take(1));

    let mut writer = InMemWriter::new();

    station.start(reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 6);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}
