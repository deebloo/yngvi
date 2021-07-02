pub struct TestReader {
    pub readings: Vec<Vec<u8>>, // A list of readings to iterate through when .read() is called
    pub current_reading: usize,
}

impl acurite::Reader for TestReader {
    fn read(&mut self, buf: &mut [u8]) -> acurite::ReadResult {
        for i in 0..=9 {
            buf[i] = self.readings[self.current_reading][i];
        }

        self.current_reading = if self.current_reading < self.readings.len() - 1 {
            self.current_reading + 1
        } else {
            0
        };

        Ok(())
    }
}
