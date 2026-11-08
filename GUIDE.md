# Installation

## Table of Contents

- [Prerequisites](#prerequisites)
- [Download Nightly Build](#download-nightly-build)
- [Build from Source](#build-from-source)
- [Configuration](#configuration)
- [Installation](#installation-1)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

- [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft server
- The server must support WASM plugins (via `wasm32-wasip2` target)

## Download Nightly Build

Download pre-built WASM binaries from GitHub releases.

### Setup

1. Download the latest nightly release:

    ```bash
    curl -L -o mine-in-abyss.wasm https://github.com/XodiumSoftware/mine-in-abyss/releases/download/nightly/mine-in-abyss-wasm32-wasip2
    ```

2. Place the `.wasm` file in your Pumpkin server's `plugins/` directory

## Build from Source

Build the plugin yourself using Rust.

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- `wasm32-wasip2` target: `rustup target add wasm32-wasip2`

### Setup

1. Clone the repository:

    ```bash
    git clone https://github.com/XodiumSoftware/mine-in-abyss.git
    cd mine-in-abyss
    ```

2. Build the plugin:

    ```bash
    cargo build --release --target wasm32-wasip2
    ```

3. The output file is at:
    ```
    target/wasm32-wasip2/release/mine-in-abyss.wasm
    ```

## Configuration

The plugin stores its configuration in `config.toml` inside the plugin's data folder.
The file is created on first run with sensible defaults.

### `config.toml`

```toml
[layers]
layers = true

[layers.config.first]
enabled = false

[layers.config.second]
enabled = false

# ... one [layers.config.<stratum>] table per layer

[layers.worlds]
enabled = false
dimension = "overworld"

[layers.worlds.strata]
first = true
second = true
third = true
fourth = true
fifth = true
sixth = true
seventh = true
```

| Key                        | Description                             | Default     |
| -------------------------- | --------------------------------------- | ----------- |
| `layers.layers`            | Master toggle for all Abyss layers      | `true`      |
| `layers.config.<stratum>.enabled` | Enable an individual layer       | `false`     |
| `layers.worlds.enabled`    | Create one Minecraft world per stratum  | `false`     |
| `layers.worlds.dimension`  | Dimension for the layer worlds          | `overworld` |
| `layers.worlds.strata.*`   | Per-stratum world toggles               | `true`      |

`<stratum>` is one of `first`, `second`, `third`, `fourth`, `fifth`, `sixth`,
`seventh`. Every layer is disabled by default; the `layers.layers` master toggle
gates whether the group is registered at all.

## Installation

1. Place `mine-in-abyss.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin will load and create `config.toml` in the plugin data folder
4. Stop the server and edit `config.toml` as needed
5. Restart the server

## Usage

The plugin currently only manages worlds: when `layers.worlds.enabled` is set, it creates
one Minecraft world per stratum on startup (`mine_in_abyss_first`, ...). Gameplay content
(the Abyss, relics, delvers, layer curses, ...) will land in `src/layers/`, under the
stratum folder it belongs to.

## Troubleshooting

### "Plugin failed to load"

- Verify your Pumpkin server supports WASM plugins
- Check server logs for detailed error messages
- Ensure the `.wasm` file is not corrupted (re-download if needed)

### "Config not loading"

- Check that the offending `.toml` file is valid TOML
- The plugin will regenerate a config file if it's invalid and fall back to defaults
- Stop the server before editing config files

### Nothing happens in-game

- The plugin is a stub right now — no gameplay modules are registered yet
- Verify the plugin loaded successfully in server logs (`Mine in Abyss loaded.`)

### Build errors

- Make sure you have the `wasm32-wasip2` target installed:
    ```bash
    rustup target add wasm32-wasip2
    ```
- Ensure you're using the latest stable Rust version
- Run `cargo update` to pull the latest `pumpkin-plugin-api` revision

---

<p align="right"><a href="#readme-top">▲</a></p>
