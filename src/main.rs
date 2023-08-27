use rtl_433::{rtl_433_source, RTL433Reader};
use weather::{InMemWriter, Station};

#[tokio::main]
async fn main() {
    let mut station = Station::new();
    let mut writer = InMemWriter { readings: vec![] };

    let source = rtl_433_source();
    let reader = RTL433Reader::new(source);

    station.start(reader, &mut writer).await;
}
