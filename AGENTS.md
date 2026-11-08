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
    - No mechanics are registered yet — this is a stub
3. **`on_unload`**: Logs farewell message

### Module System

Every feature implements the **`Mechanic`** trait (`src/modules/mechanics/mechanic.rs`):

| Method       | Purpose                                          | Default     |
| ------------ | ------------------------------------------------ | ----------- |
| `enabled()`  | Returns whether module is active                 | Required    |
| `cmds()`     | Returns `Vec<Command>` to register               | Empty vec   |
| `perms()`    | Returns `HashSet<String>` permission nodes       | Empty set   |
| `events()`   | Registers event handlers via `Context`           | No-op       |
| `register()` | Calls `events()`, registers commands/permissions | Implemented |

Modules are plain structs (not singletons) instantiated with `Default::default()` and passed to `register()` in `on_load`.

### Configuration

**`ConfigManager`** (`src/config.rs`) — TOML-backed config using the [`config`](https://crates.io/crates/config) crate:

- Master toggles live in `{data_folder}/config.toml` under a `[modules]` section
- Each module group has its own file: `mechanics.toml`, `enchantments.toml`, `recipes.toml`
- On first load: creates all files with module defaults
- On subsequent loads: merges user values with defaults and preserves extra fields
- All module configs live in a single `PluginConfig` struct in memory; modules access their section via `ConfigManager::get().map(|cm| cm.section)`
- Setting a master toggle to `false` skips loading that group's file entirely

### Active Modules

None yet — `src/` is an empty scaffold. Modules will be added under
`src/modules/mechanics/`.

### Project Structure

```
src/
├── lib.rs                    # Plugin entry point, `MineInAbyss` struct
├── config.rs                 # `ConfigManager` — TOML config load/save
└── modules/
    └── mechanics/
        └── mechanic.rs       # `Mechanic` trait + `MechanicsConfig`
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `mine_in_abyss:command.example`)
- Module configs accessed via `ConfigManager::get().and_then(|cm| cm.mechanics.example.enabled).unwrap_or(false)`
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size
- **Before checking API availability, run `cargo update` to fetch the latest `pumpkin-plugin-api` revision.** The API is git-tracked and may have added new events or types since the last local checkout.

### Code Style

**Order of items in modules:**

1. **`//!`** — Module-level documentation
2. **`use`** — imports (external, crate, std)
3. **`const`** — module constants (e.g., `PLUGIN_ID`)
4. **`struct`**/`enum`** definitions (documented fields)
5. **`impl Module`** — module trait implementation
6. **`impl EventHandler<T>`** — event handler implementations
7. **`impl OtherTrait`** — other trait implementations
8. **Type alias** — `pub type ModuleConfig = Config;`
9. **`Config` struct** — module config (at bottom)

**Documentation:**

- Module-level docs with `//!` describing module purpose and config table
- All `pub` items have `///` rustdoc comments
- Config fields documented inline
- Placeholder tables for user-facing strings

**Error Handling:**

- Use `tracing::error!` for logging, not `println!`
- Event handler failures use `.expect()` for critical registrations
- Config parsing errors logged gracefully, fall back to defaults

## Testing

- Unit tests in `src/config.rs` under `#[cfg(test)]`
- Run with: `cargo test`
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

## Adding a New Module

To add a new module, follow these steps:

1. Create new file in `src/modules/mechanics/{module}.rs`
2. Add module-level docs `//!` with description and config table
3. Define `{Module}Config` struct with `enabled: bool` + module fields
4. Implement `Default` for config with sensible defaults
5. Create `{Module}` struct deriving `Default`
6. Implement `Mechanic` trait:
    - `enabled()` — check config via `ConfigManager::get().is_some_and(|cm| cm.{section}.enabled)`
    - `events()` — register event handlers (if needed)
    - `cmds()` — return commands (if needed)
    - `perms()` — return permission nodes (if commands)
7. Implement `EventHandler<T>` for each event (if needed)
8. In `src/modules/mechanics/mechanic.rs`:
    - Add a `pub {section}: {Module}Config` field to `MechanicsConfig`
9. In `src/lib.rs`:
    - Declare the module in the `mod modules` tree
    - Instantiate and register it in `on_load`, gated on the module group toggle
10. Run `cargo build --target wasm32-wasip2` to verify

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
