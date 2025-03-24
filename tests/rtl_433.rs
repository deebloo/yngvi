use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use metrum::Temp;
use yngvi::core::{InMemWriter, Station};
use yngvi::rtl_433::RTL433Reader;

#[tokio::test]
async fn shold_read_and_record_rtl433_readings() {
    let path = "data/rtl_433.txt";
    let file = File::open(path).expect(format!("could not find file at {}", path).as_str());
    let source = BufReader::new(file).lines().take(5);

    let mut station = Station::new();

    let reader = RTL433Reader::read_from(source);
    let mut writer = InMemWriter::new();

    let _ = station.start(reader, &mut writer).await;

    // Get stored readings from the writer
    let data = writer.readings.into_iter();

    // Check writers stored weahter properties
    let rain: Vec<Option<f64>> = data.clone().map(|r| r.rain).collect();
    let rain_delta: Vec<Option<f64>> = data.clone().map(|r| r.rain_delta).collect();
    let wind_speed: Vec<Option<f64>> = data.clone().map(|r| r.wind_speed).collect();
    let wind_dir: Vec<Option<f64>> = data.clone().map(|r| r.wind_dir).collect();
    let out_temp: Vec<Option<Temp>> = data.clone().map(|r| r.out_temp).collect();
    let out_humid: Vec<Option<u8>> = data.clone().map(|r| r.out_humid).collect();
    let wind_chill: Vec<Option<Temp>> = data.clone().map(|r| r.wind_chill).collect();
    let heat_index: Vec<Option<Temp>> = data.clone().map(|r| r.heat_index).collect();
    let dew_point: Vec<Option<Temp>> = data.clone().map(|r| r.dew_point).collect();

    assert_eq!(
        rain,
        [None, Some(41.83), Some(41.83), Some(42.83), Some(42.83)]
    );

    assert_eq!(
        rain_delta,
        [Some(0.0), Some(0.0), Some(0.0), Some(1.0), Some(0.0)]
    );

    assert_eq!(
        wind_speed,
        [
            Some(3.193),
            Some(2.679),
            Some(4.222),
            Some(4.736),
            Some(2.679)
        ]
    );

    assert_eq!(
        wind_dir,
        [None, Some(90.0), Some(90.0), Some(180.0), Some(180.0)]
    );

    assert_eq!(
        out_temp,
        [
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
        ]
    );

    assert_eq!(
        out_humid,
        [Some(70), Some(70), Some(70), Some(70), Some(70)]
    );

    assert_eq!(
        wind_chill,
        [
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
            Some(Temp::from_f(55.8)),
        ]
    );

    assert_eq!(
        heat_index,
        [
            Some(Temp::from_f(54.37)),
            Some(Temp::from_f(54.37)),
            Some(Temp::from_f(54.37)),
            Some(Temp::from_f(54.37)),
            Some(Temp::from_f(54.37)),
        ]
    );

    assert_eq!(
        dew_point,
        [
            Some(Temp::from_f(46.282161213458586)),
            Some(Temp::from_f(46.282161213458586)),
            Some(Temp::from_f(46.282161213458586)),
            Some(Temp::from_f(46.282161213458586)),
            Some(Temp::from_f(46.282161213458586)),
        ]
    );
}
