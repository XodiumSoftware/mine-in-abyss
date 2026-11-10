# mine-in-abyss task runner
# Install `just` once: https://github.com/casey/just

[windows]
set shell := ["powershell.exe", "-NoProfile", "-Command"]

_default:
    @just --list

# Lint with pedantic lints enabled and warnings as errors
lint:
    cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format the project
fmt:
    cargo fmt --all

# Build the release WASM plugin
build:
    cargo build --release --target wasm32-wasip2

# Build release WASM and copy the plugin to .server/plugins/
#
# NOTE: Debug builds fail to load in the Pumpkin runtime due to a wasmtime
# locals limit in the wit-bindgen `handle-event` export. Always use release.
[unix]
build-copy:
    cargo build --release --target wasm32-wasip2
    mkdir -p .server/plugins
    cp target/wasm32-wasip2/release/mine-in-abyss.wasm .server/plugins/mine-in-abyss.wasm
    echo "Copied target/wasm32-wasip2/release/mine-in-abyss.wasm -> .server/plugins/mine-in-abyss.wasm"

[windows]
build-copy:
    cargo build --release --target wasm32-wasip2
    New-Item -ItemType Directory -Force -Path .server/plugins | Out-Null
    Copy-Item target/wasm32-wasip2/release/mine-in-abyss.wasm .server/plugins/mine-in-abyss.wasm -Force
    Write-Host "Copied target/wasm32-wasip2/release/mine-in-abyss.wasm -> .server/plugins/mine-in-abyss.wasm"

# Generate rustdoc for the WASI target
doc:
    cargo doc --no-deps --target wasm32-wasip2

# Run the full validation suite used in CI
validate: lint fmt-check build
