# Weather Station

Program for collecting and processing weather data.

HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

### Config

| Name                 | Description                                                          | Default               |
| -------------------- | -------------------------------------------------------------------- | --------------------- |
| WS_SOURCE            | determines where to pull data from. ACURITE_DISPLAY, RTL433, or FILE | DISPLAY               |
| WS_SOURCE_FILE_PATH  | path to file with source readings                                    |                       |
| WS_DEST              | INFLUXDB, STDOUT, or INMEMORY                                        | STDOUT                |
| WS_DEST_INFLUXDB_URL | influxdb url                                                         | http://localhost:8086 |
| WS_DEST_INFLUXDB_DB  | influxdb database                                                    | weather               |

### Run integration and unit tests

```BASH
cargo test --workspace
```

### build and start program

```BASH
docker compose up --build
```

## Create your own

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
