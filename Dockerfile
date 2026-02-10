# Multi-stage build for cross-platform compilation
FROM rust:1-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm

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
FROM ghcr.io/linuxserver/baseimage-ubuntu:jammy

# Image version
ARG COMMIT_SHA=unknown
ARG BUILD_TIME=unknown
ARG VERSION=unknown
LABEL org.opencontainers.image.version=${VERSION}
LABEL org.opencontainers.image.revision=${COMMIT_SHA}
LABEL org.opencontainers.image.created=${BUILD_TIME}
LABEL org.opencontainers.image.source="https://github.com/nzxl101/homedash-rs"

# Base directory
WORKDIR /app

# Install required packages
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

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
