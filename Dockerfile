# * --- Build Stage ---
FROM rust:1.60-alpine3.15 AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

RUN USER=root cargo new app

WORKDIR /usr/src/app

COPY Cargo.toml .
COPY Cargo.lock .

COPY src src

RUN cargo build --release

# * --- Running Stage ---
FROM alpine:3.15.4 as main

WORKDIR /usr/app

COPY --from=builder /usr/src/app/target/release/akashic app

COPY data data

EXPOSE 8080

CMD ["./app"]
