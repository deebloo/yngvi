use std::fs::File;
use std::io::{BufRead, BufReader};

use yngvi::core::{InMemWriter, Station};
use yngvi::rtl_433::RTL433Reader;

mod error_writer;

const PATH: &str = "data/rtl_433.txt";

#[tokio::test]
async fn should_replay_failed_writes_rtl_433() {
    let mut station = Station::new();

    let file = File::open(PATH).expect(format!("could not find file at {}", PATH).as_str());
    let source = BufReader::new(file).lines();

    let reader = RTL433Reader::read_from(source.take(5));

    let mut writer = error_writer::ErrorWriter {};

    let _ = station.start(reader, &mut writer).await;

    assert_eq!(station.retry_manager.failed_writes.len(), 5);

    let file = File::open(PATH).expect(format!("could not find file at {}", PATH).as_str());
    let source = BufReader::new(file).lines();

    let reader = RTL433Reader::read_from(source.take(1));

    let mut writer = InMemWriter::new();

    let _ = station.start(reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 6);
    assert_eq!(station.retry_manager.failed_writes.len(), 0);
}
