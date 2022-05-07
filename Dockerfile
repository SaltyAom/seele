# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# Setup tools for building
RUN rustup target add x86_64-unknown-linux-musl

# ? Create dummy project for package installation caching
RUN USER=root cargo new akashic
WORKDIR /usr/src/akashic

COPY Cargo.toml .
COPY Cargo.lock .

RUN RUSTFLAGS='-C target-cpu=native' cargo build --release

# Build project
COPY src src

RUN RUSTFLAGS='-C target-cpu=native' cargo install --target x86_64-unknown-linux-musl --path .

# * --- Running Stage ---
FROM alpine:3.15.4 as main

WORKDIR /usr/app

COPY --from=builder /usr/local/cargo/bin/akashic akashic

COPY data data

EXPOSE 8080

CMD ["./start.sh"]
