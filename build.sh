#!/bin/sh

# Build
tuono build

# Release
cargo build --release --target x86_64-unknown-linux-gnu