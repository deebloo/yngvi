# Weather Station

Dashboard for weather data collected from Acurite 5-1 weather station. In one of two ways.

1. Indoor Acurite console
   HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY
2. RTL-433

### Config

| Name                   | Description                                                  | Default               |
| ---------------------- | ------------------------------------------------------------ | --------------------- |
| WS_SOURCE              | determines where to pull data from. DISPLAY, RTL433, or FILE | DISPLAY               |
| WS_SOURCE_INFLUXDB_URL | influxdb url                                                 | http://localhost:8086 |
| WS_SOURCE_INFLUXDB_DB  | influxdb database                                            | weather               |
| WS_SOURCE_FILE_PATH    | path to file with source readings                            |                       |
| WS_DEST                | INFLUXDB, STDOUT, or INMEMORY                                | STDOUT                |

### Pull latest code

```BASH
git clone https://github.com/deebloo/acurite-weather.git
```

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
