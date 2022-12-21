FROM rust:1.66-slim AS build

WORKDIR /build

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

# ===------------------------------------------------------------------------===

FROM debian:buster-slim

WORKDIR /filedrop
COPY --from=build /build/target/release/filedrop .

ENTRYPOINT ["./filedrop"]
