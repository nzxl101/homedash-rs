# Multi-stage build for cross-platform compilation
FROM rust:slim-trixie@sha256:cc0448b41c3b7b7fea44f5dc50eacba729a56db365b65b7bd5e8a82d5b3db078 AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    jq \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | sh - && \
    apt-get install -y nodejs && \
    npm install -g pnpm \
    && rm -rf /var/lib/apt/lists/*

# Base directory
WORKDIR /app

# Frontend dependencies (cached unless package.json or lockfile change)
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Rust manifests + stub entrypoint (tuono regenerates the real one later)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p .tuono && echo 'fn main() {}' > .tuono/main.rs

# Tuono CLI (cached unless its version in package.json changes)
RUN TUONO_VERSION=$(jq -r '.dependencies.tuono' package.json) && cargo install tuono@${TUONO_VERSION}

# Dependency pre-build (cached unless manifests change)
RUN cargo build --release

# App build (only the app crate recompiles)
COPY . .
RUN tuono build && cargo build --release

# Final runtime image
FROM ghcr.io/linuxserver/baseimage-debian:trixie@sha256:2a4e23bd3c4e652c22d354d450bdd1a95892e51a47317796c8251c1a75f54aff

ARG COMMIT_SHA=unknown
ARG BUILD_TIME=unknown
ARG VERSION=unknown

ENV COMMIT_SHA=${COMMIT_SHA}
ENV BUILD_TIME=${BUILD_TIME}
ENV VERSION=${VERSION}

WORKDIR /app

HEALTHCHECK --interval=60s --timeout=30s --start-period=180s --retries=5 \
    CMD curl -f http://localhost:3000/api/health_check > /dev/null || exit 1

COPY --from=builder /app/target/release/tuono /app/tuono
COPY --from=builder /app/.tuono/ /app/.tuono/
COPY --from=builder /app/out/ /app/out/

RUN chmod +x /app/tuono

# LSIO image customization
COPY ./docker/branding /etc/s6-overlay/s6-rc.d/init-adduser/branding
RUN mkdir -p /etc/s6-overlay/s6-rc.d/init-tuono/dependencies.d && \
    mkdir -p /app/data
RUN touch /etc/s6-overlay/s6-rc.d/init-tuono/dependencies.d/init-os-end && \
    echo longrun > /etc/s6-overlay/s6-rc.d/init-tuono/type && \
    echo /etc/s6-overlay/s6-rc.d/init-tuono/run > /etc/s6-overlay/s6-rc.d/init-tuono/up && \
    printf '#!/usr/bin/with-contenv bash\n# shellcheck shell=bash\nfind /app ! -user abc -exec chown abc {} +\ncd /app && exec /app/tuono\n' > /etc/s6-overlay/s6-rc.d/init-tuono/run && \
    chmod +x /etc/s6-overlay/s6-rc.d/init-tuono/run
RUN mkdir -p /etc/s6-overlay/s6-rc.d/user/contents.d && \
    touch /etc/s6-overlay/s6-rc.d/user/contents.d/init-tuono

EXPOSE 3000
