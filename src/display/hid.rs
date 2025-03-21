use crate::display::decode::Report1;

use hidapi::HidApi;
use std::{thread, time};

pub struct HidSource {
    hid: HidApi,
    vid: u16,
    pid: u16,
}

impl HidSource {
    pub fn new(vid: u16, pid: u16) -> Result<Self, ()> {
        if let Ok(hid) = HidApi::new() {
            Ok(Self { hid, vid, pid })
        } else {
            Err(())
        }
    }
}

impl Iterator for HidSource {
    type Item = [u8; 10];

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Report1 = [1u8; 10];

        // we want to read every 18 seconds
        thread::sleep(time::Duration::from_secs(18));

        if let Ok(device) = self.hid.open(self.vid, self.pid) {
            match device.get_feature_report(&mut buf) {
                Ok(_) => Some(buf),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
