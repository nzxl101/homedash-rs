FROM node:20.15-alpine AS node-builder
RUN apk add --no-cache libc6-compat sed
WORKDIR /app

COPY package.json pnpm-lock.yaml* tuono.config.ts ./
RUN \
    sed -i 's/\/\/ \(.*\)/\1/' tuono.config.ts && \
    npm install -g corepack@latest && \
    corepack enable pnpm && \
    pnpm i --frozen-lockfile && \
    pnpm --package=tuono dlx tuono-build-config

FROM rust:1.85.1-slim AS rust-builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    curl \
    jq \
    libclang-dev \
    libglib2.0-0 \
    libglib2.0-dev \
    libssl-dev \
    musl-dev \
    musl-tools \
    nodejs \
    openssl \
    perl \
    pkg-config \
    python3 \
    python3-pip \
    sed \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV V8_FROM_SOURCE=1
ENV PYTHON=python3

COPY --from=node-builder /app/.tuono ./.tuono
COPY --from=node-builder /app/node_modules ./node_modules
COPY . ./
RUN \
    rustup target add x86_64-unknown-linux-musl && \
    sed -i 's/# openssl/openssl/' Cargo.toml && \
    TUONO_VERSION=$(jq -r '.dependencies.tuono' package.json) && \
    cargo install tuono@${TUONO_VERSION} && \
    tuono build && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM ghcr.io/linuxserver/baseimage-alpine:3.21 AS base
WORKDIR /app

COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/tuono ./
RUN chmod +x /app/tuono && \
    chown abc:abc /app/tuono && \
    chmod 755 /app && \
    chown -R abc:abc /app

COPY --from=rust-builder /app/.tuono ./
COPY --from=rust-builder /app/out ./
RUN chown -R abc:abc /app/.tuono /app/out && \
    chmod -R 755 /app/.tuono /app/out

RUN mkdir -p /app/data && \
    chown abc:abc /app/data && \
    chmod 755 /app/data

HEALTHCHECK --interval=60s --timeout=30s --start-period=180s --retries=5 \
    CMD curl -f http://localhost:3000/api/health_check > /dev/null || exit 1

ARG COMMIT_SHA=unknown
ARG BUILD_TIME=unknown
ARG VERSION=unknown
LABEL org.opencontainers.image.version=${VERSION}
LABEL org.opencontainers.image.revision=${COMMIT_SHA}
LABEL org.opencontainers.image.created=${BUILD_TIME}
LABEL org.opencontainers.image.source="https://github.com/nzxl101/homedash-rs"

EXPOSE 3000
CMD ["./tuono"]