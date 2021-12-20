# * --- Build Stage ---
FROM rust:1.57-alpine3.13 AS builder
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
COPY . .

RUN RUSTFLAGS='-C target-cpu=native' cargo install --target x86_64-unknown-linux-musl --path .

# * --- Running Stage ---
FROM scratch

COPY public /public
COPY --from=builder /usr/local/cargo/bin/akashic .

EXPOSE 8080

CMD ["./app"]