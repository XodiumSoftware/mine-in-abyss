---
name: add-layer
description: Scaffolds a new mine-in-abyss feature by extending whatever layer/extension pattern the src/ tree currently uses.
---

# Add a Feature (Layer)

Use this skill when the user wants to add a new gameplay feature to `mine-in-abyss`.

Every feature is a struct that the plugin registers with the server. The trait that defines
that contract changes as the project evolves, so **do not assume names — discover them.**

## Step 0 — Discover the current pattern (always do this first)

Before writing anything, read the codebase and answer:

1. **What is the registration trait?** Read `src/lib.rs`, note the `mod` tree and the
   `register_*` calls in `on_load`. The trait used there is the one to implement.
   Find its definition file and read it in full.
2. **What are its required and default methods?** Note the exact signatures (e.g. a stratum
   selector, `enabled()`, `events()`, `cmds()`, `perms()`, `register()`). If a method has a
   default that isn't wanted, you must override it.
3. **Where do features live?** Note the directory convention under `src/` (subfolders by
   stratum/category, flat files, etc.) and mirror it.
4. **How is config wired?** Find the config struct that holds per-feature settings
   (usually alongside the trait, plus `src/config.rs`). Note its field style (`{Name}Config`
   structs vs. plain `bool`s) and how `enabled()` reads it.
5. **How is a feature registered?** Find the exact list/vec the trait objects are collected
   into and the `on_load` guard that gates it (a config toggle).

If any of the above contradicts this skill, **trust the code**, follow it, and update this
skill afterwards so it stays accurate.

## Step 1 — Gather requirements

Ask for:

- The feature name (e.g. `relic_detector`, `CurseOfTheAbyss`).
- Which group it belongs to (stratum/category folder), if the layout uses grouping.
- Whether it needs commands, permissions, or event handlers.
- Whether its config is only an `enabled` toggle or has extra fields.

## Naming

- File/module: `snake_case`, matching the file name.
- Struct: `PascalCase`.
- Config struct: match the convention already in the codebase (commonly `{Name}Config`).
- Namespaced ids and permission nodes: use the existing `PLUGIN_ID` constant/helper found in
  `src/lib.rs` — never hardcode a namespace or re-derive it from the crate name.
- Config field name matches the file name.

## Implementation

Follow the item order documented in `AGENTS.md` (§ Code Style). In outline:

1. Module docs (`//!`) describing the feature, its config fields, and its position in the
   world. Include a config table when there are fields.
2. `use` imports.
3. `const`s (ids, permission nodes).
4. `#[derive(Default)] pub struct {Name};` — features are plain structs, not singletons.
5. `impl {Trait} for {Name}` — implement every **required** method, and override the optional
   ones the feature actually needs.
6. `impl EventHandler<T> for {Name}` blocks, registered from `events()` via the trait's
   `register_event` helper (or the equivalent the trait provides).
7. Private helpers.
8. The config struct, at the bottom.

Never log with `println!` — use `tracing::{info, debug, error}`.

## Wiring

Three places typically need updating — confirm each against the current tree:

1. **The trait's config struct** — add the feature's config field (and a `pub use` re-export
   if the file re-exports other feature configs).
2. **`src/lib.rs` `mod` tree** — declare the module in the right branch.
3. **`src/lib.rs` `on_load`** — add the feature to the registration list, gated on the
   existing group toggle, and log the enabled count if that's the established pattern.

Then update `AGENTS.md` if the tree/registration description changed.

## Verification

Use the `just` recipes where they exist, otherwise the equivalent cargo commands:

1. `just lint` (or `cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings`)
   — warnings are errors here.
2. `just fmt-check` (or `cargo fmt --all -- --check`).
3. `just build` (or `cargo build --release --target wasm32-wasip2`).
4. `just doc` (or `cargo doc --no-deps --target wasm32-wasip2`) — fix any warnings.
5. Features should default to disabled unless the project says otherwise; config files are
   regenerated with defaults on next load.
6. If a new API is needed, run `cargo update` first — `pumpkin-plugin-api` is git-tracked.

After finishing, summarize the files changed and ask the user whether to commit (per `AGENTS.md`).
