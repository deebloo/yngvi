# Weather Station

Program for collecting and processing weather data.

HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

### Install program

```bash
cargo install yngvi --features standalone
```

### Config

| Name                       | Description                                                                                      | Default |
| -------------------------- | ------------------------------------------------------------------------------------------------ | ------- |
| YNGVI_SRC                  | determines where to pull data from. ACURITE_DISPLAY, RTL_433, or FILE                            | DISPLAY |
| YNGVI_SRC_FILE_PATH        | path to file with source readings                                                                |         |
| YNGVI_DEST                 | INFLUXDB, STDOUT, WEBHOOK, or INMEMORY                                                           | STDOUT  |
| YNGVI_DEST_INFLUXDB_URL    | influxdb url                                                                                     |         |
| YNGVI_DEST_INFLUXDB_ORG    | influxdb org                                                                                     |         |
| YNGVI_DEST_INFLUXDB_BUCKET | influxdb bucket                                                                                  |         |
| YNGVI_DEST_INFLUXDB_TOKEN  | auth token for influxdb                                                                          |         |
| YNGVI_DEST_WEBHOOK_URL     | url for the webhook                                                                              |         |
| YNGVI_DEST_WEBHOOK_HEADERS | headers to add to the webhook request. Ex: YNGVI_DEST_WEBHOOK_HEADERS=Authorization:Bearer 12342 |         |

## Create your own

Any data source can be defined as `Iterator<Item = WeatherReading>` and all destinations are defined with the `Writer` trait. YNGVI comes with some prebuilt readers and writers but it should be straight forward to define new data sources and new destination without touching any of the core station logic. Give it a try!

```bash
cargo add yngvi
```

```rust
use yngvi::core::{Station, StdoutWriter, FileReader};

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
