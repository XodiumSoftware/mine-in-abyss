# Installation

## Table of Contents

- [Prerequisites](#prerequisites)
- [Download Nightly Build](#download-nightly-build)
- [Build from Source](#build-from-source)
- [Local Development Server](#local-development-server)
- [Installation](#installation)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)

> For how the layers, transitions and the curse actually work, see
> [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## Prerequisites

- [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) Minecraft server
- The server must support WASM plugins (via `wasm32-wasip2` target)

## Download Nightly Build

Download pre-built WASM binaries from GitHub releases.

### Setup

1. Download the latest nightly release:

    ```bash
    curl -L -o mine_in_abyss.wasm https://github.com/XodiumSoftware/mine-in-abyss/releases/download/nightly/mine_in_abyss-wasm32-wasip2
    ```

2. Place the `.wasm` file in your Pumpkin server's `plugins/` directory

## Build from Source

Build the plugin yourself using Rust.

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [`just`](https://github.com/casey/just) task runner
- `wasm32-wasip2` target: `rustup target add wasm32-wasip2`

### Setup

This project uses `just` for every task — run `just` with no arguments to list the recipes.

1. Clone the repository:

    ```bash
    git clone https://github.com/XodiumSoftware/mine-in-abyss.git
    cd mine-in-abyss
    ```

2. Build the plugin:

    ```bash
    just build
    ```

    (equivalent to `cargo build --release --target wasm32-wasip2`; always build in **release**,
    since debug builds fail to load in the Pumpkin runtime)

3. The output file is at:
    ```
    target/wasm32-wasip2/release/mine_in_abyss.wasm
    ```

### Local Development Server

`just build-copy` builds the plugin and copies it into `.server/plugins/`, but it does **not**
download the server for you — put the Pumpkin binary in `.server/` yourself (grab it from the
[Pumpkin releases](https://github.com/Pumpkin-MC/Pumpkin/releases/tag/nightly)), then:

```bash
just build-copy
cd .server && ./pumpkin      # pumpkin.exe on Windows
```

## Installation

1. Place `mine_in_abyss.wasm` in your Pumpkin server's `plugins/` directory
2. Start the server
3. The plugin creates one Minecraft world per layer on load, each generated empty

## Usage

The Abyss is a vertical world, so every layer is its own Minecraft world
(`mine_in_abyss_orth`, `mine_in_abyss_first`, …). All eight layers are created on
startup.

### Layers

| Layer     | World                   | Name                          |
| --------- | ----------------------- | ----------------------------- |
| `Orth`    | `mine_in_abyss_orth`    | The City on the Edge          |
| `First`   | `mine_in_abyss_first`   | Depths of the Abyss           |
| `Second`  | `mine_in_abyss_second`  | The Forest of Temptation      |
| `Third`   | `mine_in_abyss_third`   | The Great Fault               |
| `Fourth`  | `mine_in_abyss_fourth`  | The Goblets of Giants         |
| `Fifth`   | `mine_in_abyss_fifth`   | The Sea of Corpses            |
| `Sixth`   | `mine_in_abyss_sixth`   | The Capital of the Unreturned |
| `Seventh` | `mine_in_abyss_seventh` | The Final Maelstrom           |

World names are derived from the layer's ordinal, so they stay stable across releases
even if a layer is renamed.

Each layer world is a **custom map built for this project**. The plugin creates the world
containers and generates them **empty** — a void world with no terrain, blocks or
structures — so that an authored map can be added to that world without competing with
generated terrain. An empty world has no ground, so a player entering a layer that has no
map yet falls straight through it and continues to the layer below.

### Per-layer game rules

World settings are applied on every startup, so a world folder saved with different
settings cannot override them. `Orth` and `First` run without hostile mobs:

| Layer           | Game rules                 |
| --------------- | -------------------------- |
| `Orth`, `First` | `spawn-monsters` = `false` |

Minecraft's _difficulty_ is server-wide and cannot be set by a plugin, so "peaceful" is
expressed as game rules instead.

### Time of day

Each layer keeps its own clock, so they would drift apart and the sun would jump as you
descend. The plugin copies `Orth`'s clock to the other seven layers once a minute, and once
immediately on startup.

Change the time in `Orth` (e.g. `/time set night`) and the rest of the Abyss follows
within a minute. Changing it in any other layer has no lasting effect — the next sync
overwrites it. The **day number** follows as well, so `/time set 24000` in `Orth` puts
every layer on day 1. A world's _age_ is a separate counter the plugin cannot set, so it
is not shared.

### Descending and ascending

- **Descending** — falling below the world floor (`Y = -64`) in a layer teleports the player
  into the next layer down, at `Y = 318`, keeping their X and Z position. The fall distance
  so far is kept, so landing on the next layer's map hurts for the whole drop — a descent
  from high up is not a free ride.
- **Ascending** — rising above `Y = 320` teleports the player into the layer above, at
  `Y = -63`. Ascending inflicts the **Curse of the Abyss** (see below).
- **The bottom of the Abyss** — there is no layer below `Seventh`, so falling below the
  world floor (`Y = -64`) there is left to vanilla: void damage kills the player. A grey
  message warns them first.
- **The top of the Abyss** — rising above `Y = 320` in `Orth` has nowhere to go; a grey
  message tells the player so.

### Spawn points

You **cannot set a spawn point** with a bed or a respawn anchor — the Abyss is escaped on
its own terms, and a bed at the bottom would let you skip the climb. Using one tells you
so and leaves your spawn where it was.

Operators can still force one with `/spawnpoint`, which is treated as a deliberate
override rather than a player shortcut.

### Dying

When you die you respawn at **`Orth`'s world spawn**, wherever in the Abyss you fell. Death
is not a shortcut back to the top, and the Curse of the Abyss is cleared when you do.

### Monsters

Monster stats do **not** change with depth: a mob in `Seventh` is exactly as strong as the
same mob in `Orth`. Descending is still an escape, though — mobs cannot change layers, so
whatever was chasing you does not follow you down. The layer you land in holds only its own
inhabitants.

### Curse of the Abyss

The curse is **cumulative**: each ascent adds strain, so climbing back up through
several layers is worse than climbing up one. The strain is saved on the player, so it
survives a relog. It is only cleared by reaching the city above (`Orth`), or by dying.

Severity is derived from the layer's index, starting at `0` for the surface, and is
added to the player's current strain (capped at 6):

| Total strain | Symptoms                                               |
| ------------ | ------------------------------------------------------ |
| `0`          | None                                                   |
| `1`          | Nausea                                                 |
| `2`          | Nausea, weakness                                       |
| `3`          | Nausea, blindness, slowness                            |
| `4`          | Nausea, blindness, weakness, minor magic damage        |
| `5`          | Blindness, weakness, magic damage, health clamped to 1 |
| `6`          | Death                                                  |

Ascending out of `Orth` adds nothing (index `0`) — there is nowhere above it. Ascending out
of `Seventh` adds `7`, which is fatal on its own.

### Other behaviour

Entering a layer's world shows a gradient title with that layer's name.

Creative and spectator players are exempt: they are never teleported between layers
and never afflicted by the curse, so they can fly through the void and ceiling freely.

## Commands

| Command                       | Permission                    |
| ----------------------------- | ----------------------------- |
| `/abyss <layer> [player]`     | `mine_in_abyss:command.abyss` |
| `/abyss item <item>`          | `mine_in_abyss:command.abyss` |
| `/abyss summon <mob> [count]` | `mine_in_abyss:command.abyss` |

Alias: `/mia`. Requires permission level 3 (admin) or above.

`<layer>` is `orth`, `first` … `seventh` or `1` … `8`, case-insensitive. The command teleports
the player straight into the layer, so **no curse strain is added** — unlike a real ascent.

`/abyss item orthcoin` puts an **Orthcoin** in your hand. It exists to test the custom-item
path end to end — the stack is created, stamped with its identity, and the identity is read
back off the slot it landed in — and reports whether that round trip worked.

`/abyss summon <mob> [count]` spawns a custom mob at your feet, up to 16 at a time. `<mob>` is
the display name, and matching is loose: `small hammerbeak`, `Small Hammerbeak` and
`SmallHammerbeak` all work. It **refuses a mob in a layer it does not list**, so you cannot
quietly fill a layer with something that was never meant to live there.

A **player** running the command is always the one moved. `[player]` is only used from
console, where the sender is not a player:

```text
/abyss third
/abyss 5 Steve
```

Tab-completion for `<layer>` is currently unavailable: enabling it crashes the current
Pumpkin build, so it is disabled until the server is fixed.

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
