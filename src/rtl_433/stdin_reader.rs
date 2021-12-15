use acurite_core::{ReadError, ReadResult, Reader};

pub struct StdinReader {}

impl StdinReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Reader<String> for StdinReader {
    fn read(&mut self, buf: &mut String) -> ReadResult {
        if let Ok(_) = std::io::stdin().read_line(buf) {
            Ok(())
        } else {
            Err(ReadError::NoDevice)
        }
    }
}
