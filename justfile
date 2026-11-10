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
# The artifact is named after the target (`mine_in_abyss.wasm`), not the package,
# so the recipe globs for it rather than hardcoding a name.
#
# NOTE: Debug builds fail to load in the Pumpkin runtime due to a wasmtime
# locals limit in the wit-bindgen `handle-event` export. Always use release.
[unix]
build-copy:
    cargo build --release --target wasm32-wasip2
    mkdir -p .server/plugins
    sh -c 'set -- target/wasm32-wasip2/release/*.wasm; if [ "$#" -ne 1 ]; then echo "expected exactly one *.wasm in target/wasm32-wasip2/release, found $#" >&2; exit 1; fi; cp "$1" .server/plugins/; echo "Copied $1 -> .server/plugins/"'

[windows]
build-copy:
    cargo build --release --target wasm32-wasip2
    New-Item -ItemType Directory -Force -Path .server/plugins | Out-Null; & { $wasm = @(Get-ChildItem target/wasm32-wasip2/release/*.wasm); if ($wasm.Count -ne 1) { throw "expected exactly one *.wasm in target/wasm32-wasip2/release, found $($wasm.Count)" }; Copy-Item $wasm[0].FullName .server/plugins/ -Force; Write-Host "Copied $($wasm[0].FullName) -> .server/plugins/$($wasm[0].Name)" }

# Generate rustdoc for the WASI target
doc:
    cargo doc --no-deps --target wasm32-wasip2

# Run the full validation suite used in CI
validate: lint fmt-check build
