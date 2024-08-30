FROM rust:latest
LABEL org.opencontainers.image.source="https://github.com/m62624/approx_int"

WORKDIR /usr/src/approx_int
RUN cargo install cargo-tarpaulin

COPY . .