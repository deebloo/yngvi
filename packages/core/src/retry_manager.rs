use crate::WeatherReading;
use crate::Writer;

pub struct RetryManager {
    pub failed_writes: Vec<WeatherReading>,
}

impl RetryManager {
    pub fn new() -> Self {
        Self {
            failed_writes: vec![],
        }
    }

    pub fn add(&mut self, reading: WeatherReading) {
        self.failed_writes.push(reading);
    }

    pub async fn replay_failed_writes(&mut self, writer: &mut impl Writer) {
        if self.failed_writes.len() > 0 {
            println!("Replaying previously failed writes");

            let mut writes_to_clear: Vec<WeatherReading> = vec![];

            for r in &self.failed_writes {
                let res = writer.write(r).await;

                if res.is_ok() {
                    writes_to_clear.push(r.clone());
                }
            }

            for r in writes_to_clear {
                self.failed_writes.retain(|x| x.time != r.time)
            }
        }
    }
}
