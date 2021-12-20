use hidapi::HidApi;

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
            Err(ReadError::CouldNotRead)
        }
    }
}

pub struct HidReader {
    hid: HidApi,
    vid: u16,
    pid: u16,
}

impl HidReader {
    pub fn new(vid: u16, pid: u16) -> Self {
        Self {
            hid: HidApi::new().unwrap(),
            vid,
            pid,
        }
    }
}

impl Reader<[u8; 10]> for HidReader {
    fn read(&mut self, buf: &mut [u8; 10]) -> ReadResult {
        if let Ok(device) = self.hid.open(self.vid, self.pid) {
            match device.get_feature_report(buf) {
                Ok(_) => Ok(()),
                Err(_) => Err(ReadError::CouldNotRead),
            }
        } else {
            Err(ReadError::NoDevice)
        }
    }
}
