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
    - Initializes `ConfigManager` (loads/creates `config.toml`)
    - No layers are registered yet — this is a stub
3. **`on_unload`**: Logs farewell message

### Layer System

The Abyss is a vertical world, so every feature implements the **`Layer`** trait
(`src/layers/layer.rs`) and declares which stratum it belongs to.

| Method       | Purpose                                          | Default     |
| ------------ | ------------------------------------------------ | ----------- |
| `depth()`    | Returns the `Depth` stratum this layer belongs to | First       |
| `enabled()`  | Returns whether the layer is active              | Required    |
| `cmds()`     | Returns `Vec<Command>` to register               | Empty vec   |
| `perms()`    | Returns `Vec<Permission>` permission nodes       | Empty vec   |
| `events()`   | Registers event handlers via `Context`           | No-op       |
| `register()` | Calls `events()`, registers commands/permissions | Implemented |

`Depth` names the strata from the surface down, and is purely descriptive:

| Variant     | Range (m)         | Notes                     |
| ----------- | ----------------- | ------------------------- |
| `First`     | 0 – 1350          | Top of the Abyss          |
| `Second`    | 1350 – 2600       | The Forest of Temptation  |
| `Third`     | 2600 – 7000       | The Great Fault           |
| `Fourth`    | 7000 – 12000      | The Goblets of Giants     |
| `Fifth`     | 12000 – 13000     | The Sea of Corpses        |
| `Sixth`     | 13000 – 15500     | The Capital of the Unreturned |
| `Seventh`   | 15500 and below   | The Final Maelstrom       |

`Depth::start_depth()` and `Depth::from_depth()` convert between a stratum and meters.
`Depth::name()` returns the layer's effective display name — the per-layer `name` from
`config.toml`, falling back to the layer's own `Default` when blank or unset (each layer
defines its name in its `{Name}Config::default`, so there is one source of truth).
`Depth::world_name()` returns the stable world identifier and `Depth::from_world_name()`
inverts it. `Depth::gradient_colors()` reads the layer's `gradient_start` / `gradient_end`
hex strings and parses them into `RgbColor`s, and `Depth::title()` builds a gradient-styled
`TextComponent` from them. `Depth::announce_entry()` shows that title to a player.

When a player enters a layer world, `LayerEntryTitle` (in `src/lib.rs`) listens for
`PlayerChangedWorldEvent`, resolves the stratum from the destination world name, and shows
the entered-layer title. Titles are built as `TextComponent`s directly — the plugin API has
no `MiniMessage` parser, so avoid emitting `MiniMessage` strings.

Layers are plain structs (not singletons) instantiated with `Default::default()` and passed
to `register()` in `on_load`.

### World Manager

**`WorldManager`** (`src/world_manager.rs`) — creates one Minecraft world per stratum on load.

- `WorldManager::setup(context)` runs from `on_load`, right after config is loaded
- World names come from `Depth::world_name()` (`mine_in_abyss_first`, ...) so they stay
  stable even if a stratum's display name changes
- `create_world` is create-or-load: an existing world is reused, never reset
- `World` is a non-cloneable WIT resource, so only the set of ready strata is cached;
  `WorldManager::world(context, depth)` re-resolves the handle from the server
- Configured under `[layers.worlds]` in `config.toml` (`enabled`, `dimension`, `strata.*`);
  disabled by default

### Configuration

**`ConfigManager`** (`src/config.rs`) — TOML-backed config using the [`config`](https://crates.io/crates/config) crate:

- All configuration lives under a `[layers]` table in `{data_folder}/config.toml`
- Per-layer files (e.g. `layers.toml`) will be added alongside the section they belong to
- On first load: creates all files with defaults
- On subsequent loads: merges user values with defaults and preserves extra fields
- All layer configs live in a single `PluginConfig` struct in memory; layers access their own section via `cm.layers.config.for_depth(Depth::X).enabled`, or `cm.layers.config.is_enabled(Depth::X)`
- Gating is **per layer only** — there is no group-wide master toggle

### Active Layers

All seven strata exist as empty stubs, registered in `MineInAbyss::register_layers` from the
surface down. None implement gameplay yet.

| Module              | Struct    | `Depth`   |
| ------------------- | --------- | --------- |
| `src/layers/first.rs`    | `First`   | `First`   |
| `src/layers/second.rs`   | `Second`  | `Second`  |
| `src/layers/third.rs`    | `Third`   | `Third`   |
| `src/layers/fourth.rs`   | `Fourth`  | `Fourth`  |
| `src/layers/fifth.rs`    | `Fifth`   | `Fifth`   |
| `src/layers/sixth.rs`    | `Sixth`   | `Sixth`   |
| `src/layers/seventh.rs`  | `Seventh` | `Seventh` |

Each stub's `enabled()` reads its own config section (e.g. `cm.layers.config.first.enabled`).
Every layer defaults to disabled and decides for itself whether it activates.

Each layer's config struct (`FirstConfig`, `SecondConfig`, …) lives **in its own layer
file**, next to the logic that uses it. `LayersConfig` in `layer.rs` aggregates them and
offers `is_enabled(depth)` / `name(depth)` lookups so callers never match on fields.

### Project Structure

```
src/
├── lib.rs                    # Plugin entry point, `MineInAbyss` struct
├── config.rs                 # `ConfigManager` — TOML config load/save
├── world_manager.rs          # `WorldManager`, `WorldsConfig` — one world per stratum
└── layers/
    ├── layer.rs              # `Layer` trait, `Depth` strata + `LayersConfig`
    └── first.rs .. seventh.rs  # per-stratum feature modules
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `mine_in_abyss:command.example`)
- Layer configs accessed via `ConfigManager::get().and_then(|cm| cm.layers.example.enabled).unwrap_or(false)`
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size
- **Before checking API availability, run `cargo update` to fetch the latest `pumpkin-plugin-api` revision.** The API is git-tracked and may have added new events or types since the last local checkout.

### Code Style

**Order of items in layers:**

1. **`//!`** — Module-level documentation
2. **`use`** — imports (external, crate, std)
3. **`const`** — module constants (e.g., `PLUGIN_ID`)
4. **`struct`**/`enum`** definitions (documented fields)
5. **`impl Layer`** — layer trait implementation
6. **`impl EventHandler<T>`** — event handler implementations
7. **`impl OtherTrait`** — other trait implementations
8. **`Config` struct** — layer config (at bottom)

**Documentation:**

- Module-level docs with `//!` describing layer purpose, config table, and `Depth`
- All `pub` items have `///` rustdoc comments
- Config fields documented inline
- Placeholder tables for user-facing strings

**Error Handling:**

- Use `tracing::error!` for logging, not `println!`
- Event handler failures use `.expect()` for critical registrations
- Config parsing errors logged gracefully, fall back to defaults

## Testing

- The project intentionally has **no unit tests**
- Integration testing: build WASM and load in Pumpkin server

## Important Notes

- A crate name with a hyphen (`mine-in-abyss`) can't be a Minecraft namespace, so `PLUGIN_ID` is the literal `"mine_in_abyss"`
- Config file auto-created with defaults on first load
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
2. Add module-level docs `//!` with description, config table, and `Depth`
3. Define `{Layer}Config` struct with `enabled: bool` + layer fields
4. Implement `Default` for config with sensible defaults
5. Create `{Layer}` struct deriving `Default`
6. Implement `Layer` trait:
    - `depth()` — return the `Depth` stratum it belongs to
    - `enabled()` — check config via `ConfigManager::get().and_then(|cm| cm.layers.{name}.enabled).unwrap_or(false)`
    - `events()` — register event handlers (if needed)
    - `cmds()` — return commands (if needed)
    - `perms()` — return permission nodes (if commands)
7. Implement `EventHandler<T>` for each event (if needed)
8. In `src/layers/layer.rs`:
    - Add a `pub {name}: {Name}Config` field to `LayersConfig`
9. In `src/lib.rs`:
    - Declare the layer in the `mod layers` tree
    - Instantiate and register it in `on_load` via `register_layers`
10. Run `cargo build --target wasm32-wasip2` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
