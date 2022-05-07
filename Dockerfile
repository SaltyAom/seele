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
COPY . .

RUN RUSTFLAGS='-C target-cpu=native' cargo install --target x86_64-unknown-linux-musl --path .

# * --- Insatll Varnish Mod ---
FROM alpine:3.15.4 as varnish-mods

ARG BUILD_TOOLS=" \
    varnish-dev \
    automake \
    autoconf \
    libtool \
    python3 \
    py-docutils \
    make \
    git \
  "

RUN apk --update --no-cache add \
  varnish \
  $BUILD_TOOLS

RUN cd /tmp \
  && git clone https://github.com/varnish/varnish-modules \
  && cd varnish-modules \
  && git checkout 7.0 \
  && ./bootstrap \
  && ./configure \
  && make -j $(nproc) \
  && make install

# * --- Running Stage ---
FROM alpine:3.15.4 as main

WORKDIR /usr/app

RUN apk --no-cache add nodejs bash varnish nginx

COPY --from=builder /usr/local/cargo/bin/akashic akashic
COPY --from=varnish-mods /usr/lib/varnish/vmods/* /usr/lib/varnish/vmods/

COPY ./ops/varnish /etc/default/varnish
COPY ./ops/default.vcl /etc/varnish/default.vcl
COPY ./ops/default.conf /etc/nginx/conf.d/default.conf
COPY ./ops/start.sh .
COPY ./ops/parallel.sh .

COPY data data

RUN chmod parallel.sh
RUN chmod 555 start.sh

EXPOSE 3000

CMD ["/bin/sh", "start.sh"]
