use degrees::Temp;
use ws_core::{InMemWriter, Station};
use ws_display::DisplayReader;

#[tokio::test]
async fn shold_read_and_record_readings() {
    let source = vec![
        [1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
        [1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
        [1, 197, 26, 120, 0, 5, 75, 75, 3, 255],
    ]
    .into_iter();

    let reader = DisplayReader::new(source);
    let mut writer = InMemWriter::new();
    let mut station = Station::new();

    station.start(reader, &mut writer).await;

    // Get stored readings from the writer
    let data = writer.readings.into_iter();

    // Check writers stored weahter properties
    let rain: Vec<Option<f32>> = data.clone().map(|r| r.rain).collect();
    let rain_delta: Vec<Option<f32>> = data.clone().map(|r| r.rain_delta).collect();
    let wind_speed: Vec<Option<f32>> = data.clone().map(|r| r.wind_speed).collect();
    let wind_dir: Vec<Option<f32>> = data.clone().map(|r| r.wind_dir).collect();
    let wind_dir_cardinal: Vec<Option<String>> =
        data.clone().map(|r| r.wind_dir_cardinal).collect();
    let out_temp: Vec<Option<Temp>> = data.clone().map(|r| r.out_temp).collect();
    let out_humid: Vec<Option<u8>> = data.clone().map(|r| r.out_humid).collect();
    let wind_chill: Vec<Option<Temp>> = data.clone().map(|r| r.wind_chill).collect();
    let heat_index: Vec<Option<Temp>> = data.clone().map(|r| r.heat_index).collect();
    let dew_point: Vec<Option<Temp>> = data.clone().map(|r| r.dew_point).collect();

    assert_eq!(rain, [None, None, None]);
    assert_eq!(rain_delta, [Some(0.0), Some(0.0), Some(0.0)]);
    assert_eq!(wind_speed, [Some(0.0), Some(0.0), Some(0.0)]);
    assert_eq!(wind_dir, [None, None, None]);
    assert_eq!(wind_dir_cardinal, [None, None, None]);
    assert_eq!(
        out_temp,
        [
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998))
        ]
    );
    assert_eq!(out_humid, [Some(75), Some(75), Some(75)]);
    assert_eq!(
        wind_chill,
        [
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998)),
        ]
    );
    assert_eq!(
        heat_index,
        [
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998)),
            Some(Temp::F(31.499998))
        ]
    );

    assert_eq!(
        dew_point,
        [
            Some(Temp::F(24.52832)),
            Some(Temp::F(24.52832)),
            Some(Temp::F(24.52832))
        ]
    );
}
