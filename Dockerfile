# * --- Meilisearch from source ---
FROM getmeili/meilisearch:v0.29.1 as meilisearch-musl

RUN cp /bin/meilisearch /home/meilisearch

# * --- Meilisearch from source ---
FROM alpine:3.16 as meilisearch

WORKDIR /home

RUN apk add --no-cache curl

COPY ops/install-meilisearch.sh install.sh

RUN chmod 500 install.sh

RUN ./install.sh

# ? --- Index ---
FROM rust:1.64.0-slim-bullseye AS index
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
# RUN apk add --no-cache gcompat libgcc musl-dev ca-certificates cmake musl-utils libressl-dev openssl-dev zlib
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

# * --- Build Stage ---
FROM rust:1.64-alpine AS builder
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/

# Setup tools for building
# RUN apt update
# RUN apt install pkg-config libssl-dev -y
RUN apk add --no-cache musl musl-dev musl-utils gcc cmake ca-certificates libressl-dev openssl-dev gcompat libgcc libc-dev

# ? Create dummy project for package installation caching
RUN USER=root cargo new app
WORKDIR /usr/src/app

# Build project
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN rustup target add x86_64-unknown-linux-musl

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --target x86_64-unknown-linux-musl --release

# * --- Running Stage ---
FROM alpine:3.16.2

RUN apk add build-base

WORKDIR /home

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/seele ./seele
COPY --from=meilisearch-musl /home/meilisearch ./meilisearch
COPY --from=index /usr/src/app/data.ms ./data.ms

COPY data data
COPY ops/start.sh start.sh

RUN chmod 747 ./meilisearch
RUN chmod 747 ./start.sh

EXPOSE 8080

CMD ["./start.sh"]
