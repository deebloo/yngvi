use acurite_core::{ReadError, ReadResult, Reader};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Command, Stdio};

pub struct RTL433Reader {
    buf: BufReader<ChildStdout>,
}

impl RTL433Reader {
    pub fn new() -> Result<Self, Error> {
        let stdout = Command::new("sh")
            .arg("-c")
            .arg("rtl_433 -C customary -F json -M time:iso:tz")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader = BufReader::new(stdout);

        Ok(Self { buf: reader })
    }
}

impl Reader<String> for RTL433Reader {
    fn read(&mut self, buf: &mut String) -> ReadResult {
        if let Ok(_) = self.buf.read_line(buf) {
            Ok(())
        } else {
            Err(ReadError::CouldNotRead)
        }
    }
}
