#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown --release
wasm-opt -Oz -o build/foo.wasm target/wasm32-unknown-unknown/release/foo.wasm
./scripts/compile-abi.ts
