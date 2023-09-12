use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::WeatherReadingSource;

pub struct FileReader;

impl FileReader {
    pub fn new(path: &str) -> impl Iterator<Item = WeatherReadingSource> {
        let f = File::open(path).expect(format!("could not find file at {}", path).as_str());

        BufReader::new(f)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| {
                if let Ok(reading) = serde_json::from_str::<WeatherReadingSource>(&line) {
                    return Some(reading);
                }

                None
            })
    }
}
