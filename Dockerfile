FROM rust:1.67 as builder

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY packages packages

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev

RUN cargo build --release

FROM debian:stable

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev

COPY --from=builder /app/target/release/acurite /usr/local/bin/acurite

CMD ["acurite"]