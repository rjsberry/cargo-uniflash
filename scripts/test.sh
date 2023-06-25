#!/usr/bin/env bash
set -e
cd "${0%/*}"
cargo run -- -- --board launchxl-rm42 --elf rm42.elf --verify
