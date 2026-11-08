---
name: add-module
description: Scaffolds a new mine-in-abyss mechanic module with config wiring and registration.
---

# Add a Module

Use this skill when the user wants to add a new feature to `mine-in-abyss`. Every gameplay
feature is a **mechanic**: a struct implementing the `Mechanic` trait
(`src/modules/mechanics/mechanic.rs`).

| Kind      | Trait     | Location                                             | Config style              | Registered with server?             |
| --------- | --------- | ---------------------------------------------------- | ------------------------- | ----------------------------------- |
| `mechanic`| `Mechanic`| `src/modules/mechanics/{category}/{name}.rs`         | Own `{Name}Config` struct | Yes (events, commands, permissions) |

The scaffold currently has no modules — this skill is the reference for adding the first
one, and for every one after it.

## Before Writing Code

1. Ask for:
    - The module name (e.g. `CurseOfTheAbyss`, `relic_detector`) and which category folder
      fits (`entity`, `player`, `server`, `world`), creating the folder if needed.
    - Whether it needs commands, permissions, or event handlers.
    - Whether its config is just an `enabled` flag or has extra fields.
2. Reference material:
    - Mechanic trait, `register`, and `MechanicsConfig` — `src/modules/mechanics/mechanic.rs`.
    - Plugin entry point and registration order — `src/lib.rs`.
    - Config loading and merge semantics — `src/config.rs`.
    - Pumpkin API docs: https://docs.rs/pumpkin-plugin-api

## Naming

- File/module name: `snake_case` (e.g. `relic_detector.rs`).
- Struct name: `PascalCase` (e.g. `RelicDetector`).
- Config struct: `{Name}Config`.
- Plugin-namespaced ids and permission nodes use `{PLUGIN_ID}` from `src/lib.rs`
  (`"mine_in_abyss"`) — never hardcode a different namespace.
- Snake-case config field name matches the file name.

## Common Skeleton

Item order (per `AGENTS.md` code style):

1. `//!` module docs with a config table and (if applicable) a commands table
2. `use` imports
3. `const` values (e.g. `PLUGIN_ID`, permission nodes)
4. `#[derive(Default)] pub struct {Name};`
5. `impl Mechanic for {Name}` — `enabled()`, then `events()` / `cmds()` / `perms()` as needed
6. `impl EventHandler<T> for {Name}` blocks
7. Helper fns (private, above the config struct)
8. `{Name}Config` struct at the bottom

Event handlers use `impl EventHandler<T> for {Name}` +
`fn handle(&self, server: Server, event: EventData<T>) -> EventData<T>` and are registered in
`events()` via `self.register_event::<T>(context, EventPriority::Normal, true);`.

Never log with `println!` — use `tracing::{info, debug, error}`.

## Mechanic

1. Create `src/modules/mechanics/{category}/{name}.rs`.
2. `enabled()` must read the mechanic's own config section:
    ```rust
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .and_then(|cm| cm.mechanics.{snake_name}.enabled)
            .unwrap_or(false)
    }
    ```
3. Override `cmds()`/`perms()` only if there are commands. `perms()` returns `Vec<Permission>`
   paired **by index** with `cmds()`. Node pattern: `{PLUGIN_ID}:command.{name}`.
4. Define the config at the bottom of the same file:
    ```rust
    /// Configuration for the {name} mechanics module.
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct {Name}Config {
        /// Whether this module is active.
        pub enabled: bool,
        // module fields...
    }
    ```
5. Wire the config into `MechanicsConfig` in `src/modules/mechanics/mechanic.rs`:
    - Add `pub use crate::modules::mechanics::{category}::{name}::{Name}Config;` with the other
      `pub use` lines at the top.
    - Add `pub {snake_name}: {Name}Config,` with a `///` doc comment (alphabetical).
6. Wire into `src/lib.rs`:
    - Add the module to the `mod modules { mod mechanics { ... } }` tree (alphabetical),
      creating the category module if it doesn't exist yet.
    - Instantiate and register it in `on_load`, gated on `config.modules.mechanics`:
        ```rust
        use crate::mechanics::{category}::{name}::{Name};
        use crate::mechanics::mechanic::Mechanic;

        fn register_mechanics(context: &Context) {
            let mechanics: Vec<&dyn Mechanic> = vec![&{Name}];
            for mechanic in mechanics {
                mechanic.register(context);
            }
        }
        ```
      Keep using `info!` to log how many mechanics were enabled, following the existing style.

## Verification

1. Build: `cargo build --target wasm32-wasip2`
2. Lint (must pass with zero warnings — warnings are errors here):
   `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings`
3. Format: `cargo fmt --all -- --check`
4. Docs: `cargo doc --no-deps --target wasm32-wasip2`
5. New modules default to `enabled: false`; config files are auto-regenerated with defaults
   on next load, so no hand-editing of TOML is needed.
6. If the Pumpkin plugin API version matters (new events/builders), run `cargo update` first —
   the API is git-tracked.

After finishing, summarize the files changed and ask the user if they want to commit, per the
project workflow in `AGENTS.md`.
