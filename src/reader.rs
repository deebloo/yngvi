pub enum ReadError {
    NoDevice,
    CouldNotRead,
}

pub type ReadResult = Result<(), ReadError>;

pub trait Reader {
    fn read(&mut self, buff: &mut [u8]) -> ReadResult;
}
