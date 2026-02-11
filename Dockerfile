# Multi-stage build for cross-platform compilation
FROM rust:1-slim@sha256:760ad1d638d70ebbd0c61e06210e1289cbe45ff6425e3ea6e01241de3e14d08e AS builder

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
FROM ghcr.io/linuxserver/baseimage-ubuntu:jammy@sha256:7cdaaa30fc279a2677df82872669b32d841cea03efeff3918e5a347f11cef40c

# Image version
ARG COMMIT_SHA=unknown
ARG BUILD_TIME=unknown
ARG VERSION=unknown

# Base directory
WORKDIR /app

# Built-in healthcheck
HEALTHCHECK --interval=60s --timeout=30s --start-period=180s --retries=5 \
    CMD curl -f http://localhost:3000/api/health_check > /dev/null || exit 1

# Copy the built Rust binary from builder stage
COPY --from=builder /app/target/release/tuono /app/tuono
RUN chmod +x /app/tuono && \
    chown abc:abc /app/tuono && \
    chmod 755 /app && \
    chown -R abc:abc /app

# Copy server files
COPY --from=builder /app/.tuono/ /app/.tuono/
COPY --from=builder /app/out/ /app/out/
RUN chown -R abc:abc /app/.tuono /app/out && \
    chmod -R 755 /app/.tuono /app/out

# Create a directory for data
RUN mkdir -p /app/data && \
    chown abc:abc /app/data && \
    chmod 755 /app/data

# Copy branding
COPY ./branding /etc/s6-overlay/s6-rc.d/init-adduser/branding

EXPOSE 3000

CMD ["./tuono"]
