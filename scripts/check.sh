#!/usr/bin/env bash
set -e
cargo fmt -- --check
cargo check
cargo clippy -- -Dwarnings
