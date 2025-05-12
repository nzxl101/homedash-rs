#!/bin/sh

# Remove old build
rm -rf .tuono/
rm -rf ./out
rm temp.log

# Build
tuono build

# Run dev server once
# tuono dev > temp.log 2>&1 &
# pid=$!

# while ! grep -q "⚡ Tuono v[0-9]\+\.[0-9]\+\.[0-9]\+" temp.log; do
#     sleep 1
#     tail -n 5 temp.log
#     if ! kill -0 $pid 2>/dev/null; then
#         cat temp.log
#         echo "Process failed!"
#         kill $pid 2>/dev/null || true
#         exit 1
#     fi
# done

# # Kill the dev server once we see the startup message
# kill $pid 2>/dev/null || true
# lsof -ti:3000 | xargs kill -9 2>/dev/null || true

# sleep 5

# # Delete temp log
# rm temp.log

# # Build (again)
# tuono build

# Release build
cargo build --release --target x86_64-unknown-linux-gnu
#cargo run --release