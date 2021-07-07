use hidapi::HidApi;

use acurite_core::{ReadError, ReadResult, Reader};

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

impl Reader for HidReader {
    fn read(&mut self, buf: &mut [u8]) -> ReadResult {
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
