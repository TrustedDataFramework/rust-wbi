#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/foo.wasm build/foo.wasm
./scripts/compile-abi.ts