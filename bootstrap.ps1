#Requires -Version 5.1
$ErrorActionPreference = "Stop"

cargo build --all-targets
cargo test
