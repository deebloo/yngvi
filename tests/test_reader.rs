pub struct TestReader {
    pub readings: Vec<Vec<u8>>,
    pub current_reading: usize,
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
