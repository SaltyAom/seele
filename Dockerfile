# * --- Meilisearch from source ---
FROM alpine:3.16 as meilisearch

WORKDIR /home

RUN apk add curl

RUN curl -L https://install.meilisearch.com | sh

# * --- Build Stage ---
FROM rust:1.62-slim-bullseye AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
RUN apt update
RUN apt install pkg-config libssl-dev -y

# ? Create dummy project for package installation caching
RUN USER=root cargo new app
WORKDIR /usr/src/app

# Build project
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

# ? --- Indexer ---
FROM rust:1.62-slim-bullseye AS indexer
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
# RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev openssl-dev zlib
RUN apt update
RUN apt install pkg-config libssl-dev -y

# ? Create dummy project for package installation caching
RUN USER=root cargo new app
WORKDIR /usr/src/app

# Build project
COPY ops/setup/data data
COPY ops/setup/src src
COPY ops/setup/Cargo.toml Cargo.toml
COPY ops/setup/Cargo.lock Cargo.lock

COPY --from=meilisearch /home/meilisearch ./meilisearch
RUN chmod 747 ./meilisearch

RUN cargo run --release

# * --- Running Stage ---
FROM debian:11.3-slim

RUN apt update
RUN apt install pkg-config libssl-dev -y

WORKDIR /home

COPY --from=builder /usr/src/app/target/release/seele ./seele
COPY --from=meilisearch /home/meilisearch ./meilisearch
COPY --from=indexer /usr/src/app/data.ms ./data.ms

COPY ops/start.sh start.sh

RUN chmod 747 ./meilisearch
RUN chmod 747 ./start.sh

EXPOSE 8080

CMD ["./start.sh"]
