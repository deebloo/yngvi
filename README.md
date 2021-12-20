# acurite-weather

### HUGE THANKS TO [WEEWX](http://weewx.com/) FOR DECODING THE VARIOUS MESSAGES FROM THE ACURITE DISPLAY

Dashboard for weather data collected from Acurite 5-1 weather station. In one of two ways.

1) Indoor Acurite console
2) RTL-433

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
sudo ./target/release/acurite
```

![alt text](images/dashboard_2.png)
