#!/usr/bin/env bash
set -euo pipefail

cargo build --all-targets
cargo test
