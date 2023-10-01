# Weather Station

Program for collecting and processing weather data.

HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

### Install program

```bash
cargo install yngvi --features standalone
```

### Config

| Name                     | Description                                                           | Default               |
| ------------------------ | --------------------------------------------------------------------- | --------------------- |
| WS_SRC                   | determines where to pull data from. ACURITE_DISPLAY, RTL_433, or FILE | DISPLAY               |
| WS_SRC_FILE_PATH         | path to file with source readings                                     |                       |
| WS_DEST                  | INFLUXDB, INFLUXDB2, STDOUT, or INMEMORY                              | STDOUT                |
| WS_DEST_INFLUXDB_URL     | influxdb url                                                          | http://localhost:8086 |
| WS_DEST_INFLUXDB_DB      | influxdb database                                                     | weather               |
| WS_DEST_INFLUXDB2_URL    | influxdb url                                                          |                       |
| WS_DEST_INFLUXDB2_ORG    | influxdb org                                                          |                       |
| WS_DEST_INFLUXDB2_BUCKET | influxdb bucket                                                       |                       |
| WS_DEST_INFLUXDB2_TOKEN  | auth token for influxdb                                               |                       |

## Create your own

Any data source can be defined as `Iterator<Item = WeatherReading>` and all destinations are defined with the `Writer` trait. WS comes with some prebuilt readers and writers but it should be straight forward to define new data sources and new destination without touching any of the core station logic. Give it a try!

```bash
cargo add yngvi
```

```rust
use yngvi::core::{InMemWriter, Station, StdoutWriter, FileReader};

#[tokio::main]
async fn main() {
    // create a new weather station
    let mut station = Station::new();

    // define where your source data will come from
    let reader = FileReader::new("data/source.txt");

    // define where your calculated data will be written to
    let mut writer = StdoutWriter::new();

    // start your station!
    station.start(reader, writer).await
}
```
