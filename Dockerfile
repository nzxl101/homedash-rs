FROM node:20.15-alpine AS node-builder
RUN apk add --no-cache libc6-compat
WORKDIR /app

COPY package.json pnpm-lock.yaml* ./
RUN \
    npm install -g corepack@latest && \
    corepack enable pnpm && \
    pnpm i --frozen-lockfile

FROM rust:1.85.1-alpine AS rust-builder
RUN apk add --no-cache build-base lsof jq sed pkgconfig openssl openssl-dev
WORKDIR /app

COPY --from=node-builder /app/node_modules ./node_modules
COPY . ./
RUN \
    sed -i 's/# openssl/openssl/' Cargo.toml && \
    sed -i 's/\/\/ \(.*\)/\1/' tuono.config.ts && \
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