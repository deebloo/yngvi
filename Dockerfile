FROM rust:1.72.1 as builder

WORKDIR /app

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev libudev-dev

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY packages packages

RUN cargo build --release

FROM debian:bookworm

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev rtl-433

COPY --from=builder /app/target/release/ws /usr/local/bin/ws

CMD ["ws"]