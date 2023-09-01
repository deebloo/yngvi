use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::WeatherReading;

pub fn file_reader(path: &str) -> impl Iterator<Item = WeatherReading> {
    let f = File::open(path).expect(format!("could not find file at {}", path).as_str());

    BufReader::new(f)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            if let Ok(reading) = serde_json::from_str::<WeatherReading>(&line) {
                return Some(reading);
            }

            None
        })
}
