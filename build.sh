#!/usr/bin/env bash

set -euxo pipefail

(cd rust && cargo test)
(cd rust && cargo build --release --target wasm32-unknown-unknown)
(cd rust-wasm-bindgen && cargo build --release --target wasm32-unknown-unknown)
(cd web && npm run build)
