#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p release/
cp target/wasm32-unknown-unknown/release/staking_pool_test.wasm ./release/
