# acurite-weather

## HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

Dashboard for weather data collected from Acurite 5-1 weather station.
Only tested with the 5-1 weather station and [display for 5-1 weather station](https://www.acurite.com/shop-all/weather-instruments/weather-stations/5-in-1-color-weather-station-with-weather-ticker.html). (Cause that's what I have!)

Pulls data from display and populates InfluxDb with Grafana visualizations.

### Pull latest code

```BASH
git clone https://github.com/deebloo/acurite-weather.git
```

### Run integration and unit tests

```BASH
cargo test -p acurite -p acurite_core
```

### Create release build

```BASH
cargo build --release
```

### Standup infra

```BASH
docker-compose up -d
```

### Start program

```BASH
sudo ./target/release/acurite-weather
```

![alt text](images/dashboard_2.png)
