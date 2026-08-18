# Multi-stage build for cross-platform compilation
FROM rust:slim-trixie@sha256:8e8cf8f7fd54a2d23d5a743b3a03f56e26b6c774276c33fa0595111704ebb15c AS builder

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

# Copy source files
COPY . .

# Install frontend dependencies and build
RUN pnpm install --frozen-lockfile && \
    TUONO_VERSION=$(jq -r '.dependencies.tuono' package.json) && \
    cargo install tuono@${TUONO_VERSION} && \
    tuono build && \
    cargo build --release

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
