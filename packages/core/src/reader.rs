pub enum ReadError {
    NoDevice,
    CouldNotRead,
}

pub type ReadResult = Result<(), ReadError>;

pub trait Reader<T> {
    fn read(&mut self, buff: &mut T) -> ReadResult;
}

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
