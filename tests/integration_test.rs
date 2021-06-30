mod test_reader;
mod test_writer;


#[async_std::test]
async fn test_add() {
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
    let mut station = acurite::Station::new();

    for _ in 1..=3 {
        station.run(&mut reader, &mut writer).await;
    }

    assert_eq!(writer.readings.len(), 3);
}
