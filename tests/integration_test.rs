pub struct TestReader {
    readings: Vec<Vec<u8>>,
    current_reading: usize,
}

impl acurite::Reader for TestReader {
    fn read(&mut self, buf: &mut [u8]) -> acurite::ReadResult {
        for i in 0..=9 {
            buf[i] = self.readings[self.current_reading][i];
        }

        self.current_reading += 1;

        Ok(())
    }
}

pub struct TestWriter {
    readings: Vec<acurite::WeatherReading>,
}

#[async_trait::async_trait]
impl acurite::Writer for TestWriter {
    async fn write(&mut self, weather_reading: &acurite::WeatherReading) -> Result<(), ()> {
        self.readings.push(weather_reading.clone());

        Ok(())
    }
}

#[async_std::test]
async fn test_add() {
    let mut reader = TestReader {
        current_reading: 0,
        readings: vec![
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
            vec![1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
        ],
    };

    let mut writer = TestWriter { readings: vec![] };
    let mut station = acurite::Station::new();

    for _ in 1..=3 {
        station.run(&mut reader, &mut writer).await;
    }

    assert_eq!(writer.readings.len(), 3);
}
