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

# Copy the pre-built Rust binary from the local filesystem
COPY ./target/x86_64-unknown-linux-gnu/release/tuono /app/tuono
RUN chmod +x /app/tuono && \
    chown abc:abc /app/tuono && \
    chmod 755 /app && \
    chown -R abc:abc /app

# Copy server files
COPY ./.tuono/ /app/.tuono/
COPY ./out/ /app/out/
RUN chown -R abc:abc /app/.tuono /app/out && \
    chmod -R 755 /app/.tuono /app/out

# Create a directory for data
RUN mkdir -p /app/data && \
    chown abc:abc /app/data && \
    chmod 755 /app/data

EXPOSE 3000

CMD ["./tuono"]