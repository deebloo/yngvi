# Weather Station

Program for collecting and processing weather data.

HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

### Config

| Name                 | Description                                                           | Default               |
| -------------------- | --------------------------------------------------------------------- | --------------------- |
| WS_SRC               | determines where to pull data from. ACURITE_DISPLAY, RTL_433, or FILE | DISPLAY               |
| WS_SRC_FILE_PATH     | path to file with source readings                                     |                       |
| WS_DEST              | INFLUXDB, STDOUT, or INMEMORY                                         | STDOUT                |
| WS_DEST_INFLUXDB_URL | influxdb url                                                          | http://localhost:8086 |
| WS_DEST_INFLUXDB_DB  | influxdb database                                                     | weather               |

### Run integration and unit tests

```BASH
cargo test --workspace
```

### build and start program

```BASH
docker compose up --build
```

## Create your own

Any data source can be defined as `Iterator<Item = WeatherReading>` and all destinations are defined with the `Writer` trait. This means that it is straight forward to define new data sources and new destination without touching any of the core station logic. Give it a try!

```rust
use ws_core::{InMemWriter, Station, StdoutWriter, WeatherReading, FileReader};

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
