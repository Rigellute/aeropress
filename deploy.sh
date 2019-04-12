#!/usr/bin/env bash

# Exit on error
set -e

cargo test
cargo web deploy --release
cp now.json target/deploy
now target/deploy




