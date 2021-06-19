FROM rust:buster

RUN apt-get update -y
RUN apt-get install -y pkg-config libusb-1.0-0-dev
RUN pkg-config --libs --cflags libusb-1.0

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release