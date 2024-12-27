FROM rust:slim

RUN apt-get update && apt-get install -y \
    build-essential \
    musl-tools \
    curl \
    pkg-config \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

ENV OPENSSL_DIR=/usr 
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig 
ENV PKG_CONFIG_SYSROOT_DIR=/usr
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_DIR=/usr

WORKDIR /usr/src/app

CMD [ "cargo", "build", "--release" ]
