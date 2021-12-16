pub enum ReadError {
    NoDevice,
    CouldNotRead,
}

pub type ReadResult = Result<(), ReadError>;

pub trait Reader<T> {
    fn read(&mut self, buff: &mut T) -> ReadResult;
}
