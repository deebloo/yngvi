mod test_reader;
mod test_writer;

#[tokio::test]
async fn should_replay_failed_writes() {
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
    let mut station = acurite_core::Station::new();

    // Error Out 3 times
    for _ in 1..=3 {
        station.run(&mut reader, &mut error_writer).await;
    }

    assert_eq!(station.failed_writes.len(), 3);

    station.run(&mut reader, &mut writer).await;

    assert_eq!(writer.readings.len(), 4);
    assert_eq!(station.failed_writes.len(), 0);
}
