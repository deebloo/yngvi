# Use a base image with the desired Linux distribution (e.g., Debian)
FROM rust as builder

WORKDIR /app

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY packages packages

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update -y
RUN apt-get install -y libusb-1.0-0-dev

COPY --from=builder /app/target/release/acurite /usr/local/bin/acurite

CMD ["acurite"]