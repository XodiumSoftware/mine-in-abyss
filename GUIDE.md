# Installation

## Table of Contents

- [Prerequisites](#prerequisites)
- [Download Nightly Build](#download-nightly-build)
- [Build from Source](#build-from-source)
- [Installation](#installation)
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

## Installation

1. Place `mine-in-abyss.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin creates one Minecraft world per layer on load

## Usage

The Abyss is a vertical world, so every layer is its own Minecraft world
(`mine_in_abyss_first`, `mine_in_abyss_second`, …). All seven layers are created on
startup.

### Layers

| Layer     | World                     | Name                          |
| --------- | ------------------------- | ----------------------------- |
| `First`   | `mine_in_abyss_first`     | The Top of the Abyss          |
| `Second`  | `mine_in_abyss_second`    | The Forest of Temptation      |
| `Third`   | `mine_in_abyss_third`     | The Great Fault               |
| `Fourth`  | `mine_in_abyss_fourth`    | The Goblets of Giants         |
| `Fifth`   | `mine_in_abyss_fifth`     | The Sea of Corpses            |
| `Sixth`   | `mine_in_abyss_sixth`     | The Capital of the Unreturned |
| `Seventh` | `mine_in_abyss_seventh`   | The Final Maelstrom           |

World names are derived from the layer's ordinal, so they stay stable across releases
even if a layer is renamed.

### Descending and ascending

- **Descending** — falling below `Y = -128` in a layer teleports the player into the
  next layer down, at `Y = 318`, keeping their X and Z position.
- **Ascending** — rising above `Y = 320` teleports the player into the layer above, at
  `Y = -127`. Ascending inflicts the **Curse of the Abyss** (see below).
- **The bottom of the Abyss** — falling below `Y = -128` in `Seventh` kills the player
  in the void.

### Curse of the Abyss

The curse is **cumulative**: each ascent adds strain, so climbing back up through
several layers is worse than climbing up one. The strain is saved on the player, so it
survives a relog. It is only cleared by reaching the surface (`First`), or by dying.

Severity is derived from the layer's index, starting at `0` for the surface, and is
added to the player's current strain (capped at 6):

| Total strain | Symptoms                                              |
| ------------ | ----------------------------------------------------- |
| `0`          | None                                                  |
| `1`          | Nausea                                                |
| `2`          | Nausea, weakness                                      |
| `3`          | Nausea, blindness, slowness                           |
| `4`          | Nausea, blindness, weakness, minor magic damage       |
| `5`          | Blindness, weakness, magic damage, health clamped to 1 |
| `6`          | Death                                                 |

Ascending out of `First` adds nothing (index `0`). Ascending out of `Seventh` adds
`6`, which is fatal on its own.

### Other behaviour

Entering a layer's world shows a gradient title with that layer's name.

## Troubleshooting

### "Plugin failed to load"

- Verify your Pumpkin server supports WASM plugins
- Check server logs for detailed error messages
- Ensure the `.wasm` file is not corrupted (re-download if needed)

### Nothing happens in-game

- The layer stubs have no gameplay content yet — only worlds, layer transitions, entry
  titles and the curse are implemented
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
