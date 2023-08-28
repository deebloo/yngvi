FROM rust:1.72.0 as builder

WORKDIR /app

RUN apt-get --allow-unauthenticated update -y
RUN apt-get --allow-unauthenticated install -y libusb-1.0-0-dev libudev-dev

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY packages packages

RUN cargo build --release

# FROM debian:bullseye-slim

# RUN apt-get update -y --allow-unauthenticated
# RUN apt-get install -y libusb-1.0-0-dev rtl-433

# COPY --from=builder /app/target/release/weather_station /usr/local/bin/weather_station

# CMD ["weather_station"]