# Installation

How to build, install and play `mine-in-abyss`. For the design behind the layers, see
[`ARCHITECTURE.md`](ARCHITECTURE.md).

## Prerequisites

- A [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) server with WASM plugin support
- To build: [Rust](https://rustup.rs/) and [`just`](https://github.com/casey/just), plus
  `rustup target add wasm32-wasip3`

## Install

Download the nightly build and drop it in your server's `plugins/` directory:

```bash
curl -L -o mine_in_abyss.wasm https://github.com/XodiumSoftware/mine-in-abyss/releases/download/nightly/mine_in_abyss-wasm32-wasip2
```

Or build from source with `just build`, which writes
`target/wasm32-wasip3/release/mine_in_abyss.wasm`. Always build in **release**: debug builds
fail to load in the Pumpkin runtime.

For a local server, `just build-copy` builds and copies into `.server/plugins/`. It does not
download Pumpkin for you — put the binary in `.server/` first, then run it from there.

On load the plugin creates one empty world per layer. Each is meant to hold an authored map,
so an empty one has no ground and a player entering it falls straight through into the layer
below.

## Layers

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

World names follow the layer's ordinal, so they stay stable even if a layer is renamed.

## Playing

- **Descending** — falling below `Y = -64` moves you into the next layer at `Y = 318`,
  keeping your X and Z. The fall distance carries, so a long drop still hurts on landing.
- **Ascending** — rising above `Y = 320` moves you into the layer above at `Y = -63` and
  inflicts the Curse of the Abyss.
- **The ends of the Abyss** — below `Seventh` is vanilla void damage; above `Orth` there is
  nowhere to go. Both warn you first.
- **Entering a layer** shows a gradient title with its name.
- **Creative and spectator** players are exempt from transitions and the curse.

### Time

Every layer follows `Orth`'s clock, synced once a minute and once on startup. Set the time in
`Orth` and the rest follows; setting it anywhere else is overwritten on the next sync. The day
number follows too, but a world's _age_ is not shared.

### Spawn points and death

You cannot set a spawn point with a bed or a respawn anchor — a bed at the bottom of the Abyss
would let you skip the climb. `/spawnpoint` still works for operators. On death you respawn at
`Orth`'s world spawn with the curse cleared.

### Curse of the Abyss

Strain is cumulative and saved on the player, so climbing several layers at once is worse than
one. Only reaching `Orth` or dying clears it.

| Total strain | Symptoms                                 |
| ------------ | ---------------------------------------- |
| `0`          | None                                     |
| `1`          | Nausea                                   |
| `2`          | Nausea, weakness                         |
| `3`          | Nausea, blindness, slowness              |
| `4`          | Nausea, blindness, weakness              |
| `5`          | Blindness, weakness, health clamped to 1 |
| `6`          | Death                                    |

## Commands

| Command                   | Permission                    |
| ------------------------- | ----------------------------- |
| `/abyss <layer> [player]` | `mine_in_abyss:command.abyss` |

Alias `/mia`, requires permission level 3. `<layer>` is `orth` … `seventh` or `1` … `8`. It
teleports directly, so it adds no curse strain. A player running it is always the one moved;
`[player]` is only used from console. Tab-completion is unavailable because enabling it
crashes the current Pumpkin build.

There is no command to give a custom item or summon a custom mob yet; custom mobs appear in
the layers through normal spawning.

## Troubleshooting

- **"Plugin failed to load"** — confirm the server supports WASM plugins and check the logs
  for the cause; re-download the `.wasm` if in doubt.
- **Nothing happens in-game** — only worlds, transitions, titles and the curse exist; layer
  stubs have no gameplay content. Confirm `Mine in Abyss loaded.` appears in the logs.
- **Build errors** — check the wasm target is installed (`rustup target add wasm32-wasip3`)
  and that Rust is current.
