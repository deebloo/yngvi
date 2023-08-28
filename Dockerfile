FROM rust:1.72.0 as builder

WORKDIR /app

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY packages packages

RUN cargo build --release

FROM debian

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev rtl-433

COPY --from=builder /app/target/release/weather_station /usr/local/bin/weather_station

CMD ["weather_station"]