# mine-in-abyss — Claude Code Context

## Project at a Glance

- **Name:** mine-in-abyss
- **Type:** Pumpkin Minecraft plugin (WASM)
- **Purpose:** The recreation of 'Made In Abyss' into Minecraft.
- **MC Version:** Latest Pumpkin (tracks nightly)
- **Language:** Rust (Edition 2024)
- **Build Tool:** Cargo
- **Output:** `mine-in-abyss.wasm` (WASI Preview 2)
- **Target:** `wasm32-wasip2`
- **Worlds:** One Minecraft world per layer, each a **custom hand-built map** authored
  for this project — not generated terrain. See [Each layer is a hand-built map](#each-layer-is-a-hand-built-map).

## APIs & Tools

| Category          | Technology                                                  | Purpose                          |
| ----------------- | ----------------------------------------------------------- | -------------------------------- |
| **Core API**      | [pumpkin-plugin-api](https://github.com/Pumpkin-MC/Pumpkin) | Minecraft server plugin API      |
| **Language**      | Rust 2024                                                   | Systems language                 |
| **Build Tool**    | Cargo                                                       | Build automation                 |
| **Serialization** | serde + toml                                                | Config serialization             |
| **Config**        | config                                                      | TOML config with merge semantics |
| **Logging**       | tracing                                                     | Structured logging               |
| **Docs**          | rustdoc (via `cargo doc`)                                   | API documentation                |

### Pumpkin API Resources

- **Repository**: https://github.com/Pumpkin-MC/Pumpkin
- **Plugin API**: https://github.com/Pumpkin-MC/Pumpkin/tree/master/pumpkin-plugin-api
- **Plugin Knowledge Base**: https://book.vypal.me/

### Pumpkin API Notes

- Plugin entry point implements `Plugin` trait
- Events use `EventHandler<T>` trait with `EventPriority`
- Commands use `CommandHandler` trait with `CommandNode` builder pattern
- Plugin registered via `register_plugin!(PluginName)` macro
- WASM target requires `wasm32-wasip2` toolchain

## Quick Commands

Build tasks run through [just](https://github.com/casey/just) (cross-platform; see the `justfile` for all recipes).

```bash
# Build the WASM plugin (debug), fetch the Pumpkin nightly server, and copy to .server/plugins/
just build-copy

# Build the WASM plugin (release)
just build

# Generate documentation
cargo doc --no-deps --target wasm32-wasip2

# Lint with pedantic lints enabled; every warning is treated as an error.
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
```

## Architecture Overview

### Entry Point

**`MineInAbyss`** — implements `Plugin` from `pumpkin_plugin_api`:

1. **Registration**: Via `register_plugin!(MineInAbyss)` macro
2. **`on_load`**:
    - Registers the event handlers (see [Events](#events))
    - Creates one world per layer via `Worlds::setup`
3. **`on_unload`**: Logs farewell message

### Layer System

The Abyss is a vertical world, so every feature implements the **`Layer`** trait
(`src/layers/layer.rs`) and declares which stratum it belongs to.

| Method    | Purpose                                       |
| --------- | --------------------------------------------- |
| `id()`    | Returns the `Section` this layer lives in     |
| `name()`  | Returns the layer's display name              |
| `color()` | Returns the two gradient stops of its palette |

`Section` (`src/section.rs`) names the strata from the surface down and is the
single enum everything derives from:

| Variant   | `id()`    | World                   | Notes                         |
| --------- | --------- | ----------------------- | ----------------------------- |
| `First`   | `first`   | `mine_in_abyss_first`   | The Top of the Abyss          |
| `Second`  | `second`  | `mine_in_abyss_second`  | The Forest of Temptation      |
| `Third`   | `third`   | `mine_in_abyss_third`   | The Great Fault               |
| `Fourth`  | `fourth`  | `mine_in_abyss_fourth`  | The Goblets of Giants         |
| `Fifth`   | `fifth`   | `mine_in_abyss_fifth`   | The Sea of Corpses            |
| `Sixth`   | `sixth`   | `mine_in_abyss_sixth`   | The Capital of the Unreturned |
| `Seventh` | `seventh` | `mine_in_abyss_seventh` | The Final Maelstrom           |

It derives `EnumIter` + `IntoStaticStr` (with `serialize_all = "lowercase"`), so
`id()` is just `self.into()` — variant names are never hand-written twice. Built on
that:

- `world()` — `format!("{PLUGIN_ID}_{}", self.id())`
- `next()` / `prev()` — neighbouring strata, `None` at the ends
- `index()` — depth index, `0` at the surface
- `curse()` — Curse of the Abyss severity, derived from `index()`

Display names and gradient colours are **not** on `Section`; each layer provides them
through the `Layer` trait.

Layers are zero-sized plain structs (`First`, `Second`, …) exposed through
`LAYERS: [&dyn Layer; 7]` in `src/lib.rs`, which is the single source of truth for
registration, world creation and world-name lookup (`layer_for_world`).

### Events

Handlers live in `src/events/`, declared inline in the `mod events` block in `lib.rs`:

| Handler        | Event                     | Behaviour                                                                |
| -------------- | ------------------------- | ------------------------------------------------------------------------ |
| `LayerJoin`    | `PlayerChangedWorldEvent` | Shows the entered layer's gradient title; cures the curse on the surface |
| `LayerLeave`   | `PlayerMoveEvent`         | Void/ceiling detection, teleport, curse                                  |
| `LayerRespawn` | `PlayerRespawnEvent`      | Clears the curse strain                                                  |

`LayerLeave` compares against the thresholds shared by every section
(`Section::VOID_Y = -128.0`, `Section::MAX_Y = 320.0` — each layer is its own
full-height world, so they all use the vanilla Overworld limits): below the void → next
layer down at `entry_y()` (`MAX_Y - 2.0`); above the ceiling → previous layer up at
`exit_y()` (`VOID_Y + 1.0`). Both keep X/Z. Falling out of `Seventh` (no `next()`) is
left alone — vanilla void damage kills the player. Ascending calls `curse::apply`, which
adds to the player's accumulated strain.

**Creative and spectator players are skipped**: a player in `GameMode::Creative` or
`GameMode::Spectator` returns from the handler untouched, so they can fly through the
void and ceiling without being teleported or cursed.

**Transitions are one-shot.** `PlayerMoveEvent` fires on every movement, so without a
latch a player inside the void or ceiling band would be teleported (and cursed) on every
tick. `utils::{begin_transition, end_transition}` keep a thread-local set of player ids
currently transitioning; the second call returns `false` and the handler bails. The latch
is released immediately after the move and is deliberately **not** persistent, so a stale
entry can never lock a player out of transitions.

A successful transition also returns the move event with `cancelled: true`, so the
server does not apply the original `to_position` — which would move the player back into
the band of the world they just left.

**Leaving the Abyss at either end tells the player.** Rising above `First` or falling
below `Seventh` has nowhere to go, so `LayerLeave` sends a grey system message instead of
silently doing nothing.

Titles are built as `TextComponent`s directly — the plugin API has no `MiniMessage`
parser, so **never emit `MiniMessage` strings**. `color_named` and friends are
consuming builders: reassign the result (`let t = TextComponent::text(..).color_named(..)`)

### Commands

Commands live in `src/commands/<name>.rs` and are declared inline in the `mod commands`
block in `lib.rs`. Each module exposes `register(context)`, which is called from
`on_load`.

| Command  | Aliases | Permission                    | Behaviour                                                    |
| -------- | ------- | ----------------------------- | ------------------------------------------------------------ |
| `/abyss` | `/mia`  | `mine_in_abyss:command.abyss` | Moves a player into a layer without the curse (ops level 3+) |

`/abyss <layer> [player]` accepts `first` … `seventh` or `1` … `7`, case-insensitive,
and defaults the target to the sender. It teleports straight into the section, so no
curse strain is added — unlike a real ascent.

Registration goes through the raw `Context::register_command(command, node)` plus
`Context::register_permission(&Permission)`. Permission nodes are built from
`PLUGIN_ID` (`abyss::permission_node()`), never hand-formatted. Handler structs
implement `pumpkin_plugin_api::commands::CommandHandler` and return `Ok(1)` on success.

### Curse of the Abyss

`src/events/curse.rs` stores a `curse` strain byte on the player via
`PersistentDataHolder` (namespaced NBT, persisted to disk), so it survives relogs.

- `strain(player)` / `cure(player)` read and clear it
- `apply(player, from)` adds `from.curse()` to the strain, saturating at `MAX_STRAIN`
  (6), then inflicts the symptoms for the **total**
- Ascending several layers in a row therefore compounds: the curse is cumulative
- Reaching `First`, or respawning, calls `cure`
- Strain 6 is fatal — `inflict` kills rather than applying effects

### Worlds

**`Worlds`** (`src/worlds.rs`) — creates one Minecraft world per layer on load.

- `Worlds::setup(context, layers)` runs from `on_load`
- World names come from `Section::world()`, so they stay stable across renames
- `create_world` is create-or-load: an existing world is reused, never reset
- `World` is a non-cloneable WIT resource, so only the set of ready world names is
  cached in a `thread_local`; `Worlds::world` re-resolves the handle from the server
- A failure for one layer is logged and does not stop the others

#### Each layer is a hand-built map

Every layer world is a **custom map made specifically for this project** — an authored
region with real terrain, structures and points of interest, not a generated world and
not a plain void platform. This is the central assumption behind the whole design, so
keep it in mind whenever you touch transitions or placement:

- `create_world` only takes a name and dimension — there is no generator, seed or
  level-type argument. The map **is** the world folder on disk, so shipping a layer means
  shipping its world data.
- Create-or-load means a map that already exists on disk is reused untouched. Never write
  code that resets, regenerates or overwrites layer world data on startup.
- Because the terrain is authored, **do not assume there is ground at a given X/Z**. The
  transition code preserves X/Z and picks a Y from `Section::entry_y()` / `exit_y()`;
  whether that lands on solid ground depends entirely on the map at those coordinates.
- If a layer needs a guaranteed arrival point, that must be map data (a spawn point or a
  marker) or an explicit safe-position lookup — never a hardcoded coordinate.
- Void and ceiling thresholds (`Section::VOID_Y` / `Section::MAX_Y`) are vanilla world
  limits, which is why every layer can be a full-height world without interfering with
  its neighbours.

### Configuration

**None.** The TOML config system was removed; there is no `config.rs`, no
`config.toml`, and layers do not gate on config.

### Active Layers

All seven layers exist as stubs (identity, name, gradient colour) with no gameplay yet.

| Module                  | Struct    | `Section` |
| ----------------------- | --------- | --------- |
| `src/layers/first.rs`   | `First`   | `First`   |
| `src/layers/second.rs`  | `Second`  | `Second`  |
| `src/layers/third.rs`   | `Third`   | `Third`   |
| `src/layers/fourth.rs`  | `Fourth`  | `Fourth`  |
| `src/layers/fifth.rs`   | `Fifth`   | `Fifth`   |
| `src/layers/sixth.rs`   | `Sixth`   | `Sixth`   |
| `src/layers/seventh.rs` | `Seventh` | `Seventh` |

Each layer implements `Layer` with its own `id()`, `name()` and `color()`. Names and
colours are hardcoded per layer, next to the layer they describe.

### Project Structure

```
src/
├── lib.rs              # `PLUGIN_ID`, `LAYERS`, `layer_for_world`, `Plugin` impl
├── section.rs          # `Section` enum + id/world/next/prev/index/curse,
│                       # plus shared `VOID_Y`/`MAX_Y` and entry/exit heights
├── utils.rs            # Shared helpers (`ignores_transitions`, `begin_transition`)
├── worlds.rs           # `Worlds` — one world per layer
├── commands/
│   └── abyss.rs        # `/abyss` — ops teleport into a layer, curse-free
├── events/
│   ├── curse.rs        # Curse of the Abyss: strain storage + severity ladder
│   ├── layer_join.rs   # `LayerJoin` — entry title, cure on the surface
│   ├── layer_leave.rs  # `LayerLeave` — void/ceiling teleport + curse
│   └── layer_respawn.rs# `LayerRespawn` — clears the curse on respawn
└── layers/
    ├── layer.rs        # `Layer` trait
    └── first.rs .. seventh.rs
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- **No `//` line comments.** Explain intent in rustdoc (`///`, `//!`) and in identifier
  and function names instead. Only `///` and `//!` comments belong in `src/`.
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `mine_in_abyss:command.example`)
- No config system
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size
- **Before checking API availability, run `cargo update` to fetch the latest `pumpkin-plugin-api` revision.** The API is git-tracked and may have added new events or types since the last local checkout.

### Code Style

**Order of items in layers:**

1. **`//!`** — Module-level documentation
2. **`use`** — imports (external, crate, std)
3. **`const`** — module constants (e.g., `PLUGIN_ID`)
4. **`struct`**/**`enum`** definitions (documented fields)
5. **`impl Layer`** — layer trait implementation
6. **`impl EventHandler<T>`** — event handler implementations
7. **`impl OtherTrait`** — other trait implementations

**Documentation:**

- Module-level docs with `//!` describing the layer's purpose and its Y range
- All `pub` items have `///` rustdoc comments

**Error Handling:**

- Use `tracing::{info, warn, error}!` for logging, not `println!`
- Event handler failures use `.expect()` for critical registrations

## Testing

- The project intentionally has **no unit tests**
- Integration testing: build WASM and load in Pumpkin server

## Important Notes

- A crate name with a hyphen (`mine-in-abyss`) can't be a Minecraft namespace, so `PLUGIN_ID` is the literal `"mine_in_abyss"`
- There is no config file; the plugin has no persisted settings
- WASM output must be copied to server's `plugins/` folder
- Pumpkin server must support WASM plugins (WASI Preview 2)
- Plugin API is unstable and may change

## Claude Code Workflow

### Task Management

**When creating tasks:**

- Number tasks in the name (e.g., "1. Add MOTD module", "2. Fix chat filter")

**After completing each task:**

- Ask the user if they want to git commit the changes or adjust before committing

**When all tasks in a worktree are complete:**

- Ask the user if they want to git publish (push) the changes or adjust before publishing

### After Making Edits

**Always update documentation when code changes:**

1. **rustdoc comments** — Add/update if you:
    - Add new modules or public APIs
    - Change config fields or placeholders
    - Add commands or events
    - **Run `cargo doc --no-deps --target wasm32-wasip2`** to verify

**Rule of thumb:** If a code change would confuse someone reading the docs, update the docs.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:

- **rust.yml** — Lints, builds the WASM plugin on push/PR, uploads the artifact and publishes the nightly release
- **enforce_pr_title.yml** — Validates PR titles follow conventional commits

## Adding a New Layer

To add a new layer, follow these steps:

1. Create new file in `src/layers/{layer}.rs`
2. Add module-level docs `//!` with a description and the layer's Y range
3. Create the `{Layer}` struct deriving `Debug, Default, Clone, Copy`
4. Implement `Layer`:
    - `id()` — return the `Section` variant it belongs to
    - `name()` — return the layer's display name
    - `color()` — return the two gradient stops as `RgbColor`s
5. Implement `EventHandler<T>` for each event (if needed), and register it in
   `MineInAbyss::register_events`
6. In `src/section.rs`: add a variant to the `Section` enum (keep it in depth order —
   `next`/`prev`/`index`/`curse` all derive from the variant order)
7. In `src/lib.rs`:
    - Declare the layer in the `mod layers` tree
    - Add it to `LAYERS` **in depth order**
8. Run `cargo clippy --target wasm32-wasip2 -- -W clippy::pedantic -D warnings` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
