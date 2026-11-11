---
name: bootstrap-audit
description: Audits mine-in-abyss registration wiring so every module is declared, every event handler and command is registered, every layer is fully described, and every mob and item kind reaches its catalogue.
---

# Bootstrap Audit

Use when the user asks to verify plugin wiring, when something is missing at runtime, or
after adding or renaming several modules. Things easily end up declared but never registered.

This is a **read-only** audit. Report findings in a table; only fix things if the user asks.

**Derive the checks from the code, don't hardcode them.** Read `src/lib.rs`, `src/layer.rs`,
`src/commands/` and `src/items/` first, then build the inventory with searches
(`find src -name '*.rs'`) rather than relying on a remembered folder list.

There is **no config system and no feature/enabled flags**. An earlier version of this skill
audited `config.rs`, `enabled()` toggles and a `cmds()`/`perms()` trait; none of that exists.

## What To Check

### 1. Module declaration vs. file existence

- Every `.rs` file under `src/` is reachable from a `mod` declaration in `src/lib.rs`
  (directly, or through `mod commands` / `mod events`).
- The `mod` blocks match the on-disk layout: flag missing declarations and stale ones pointing
  at files that no longer exist.

### 2. Event handlers registered

- Every module in `mod events` that defines an `impl EventHandler<E>` also has an `impl Event`
  and appears in exactly one entry of the array in `MineInAbyss::events` (`src/lib.rs`).
- Flag handler modules that are declared but absent from that array, and array entries whose
  handler type no longer exists.
- `curse.rs` is a helper module, not a handler — it has no `impl EventHandler` and is
  correctly not in the array. Any other non-handler module in `mod events` should be
  questioned.

### 3. Commands registered

- Every module in `mod commands` has an `impl Command` (the trait in `src/commands/command.rs`)
  and appears in exactly one entry of the array in `MineInAbyss::commands` (`src/lib.rs`).
- The command's `register` calls **both** `register_command` and `register_permission`, using
  `Self::permission_node()` rather than a hand-written string.
- The node is derived from `Command::names()[0]`, so a rename changes it automatically — flag
  any literal `"mine_in_abyss:command.…"`.

### 4. Layer completeness

- `Layer::ALL` lists every variant of the enum, in depth order.
- Every variant has an arm in `Layer::name()`, `Layer::color()` and `Layer::game_rules()`.
  The compiler enforces this for exhaustive `match`; flag any that used a `_` arm to dodge it.
- `LAYERS` in `src/lib.rs` is `Layer::ALL` — flag any other literal list of layers.

### 5. Custom mob and item catalogues

A mob or item kind is not a module like the others: it has a catalogue entry that a rename or
a forgotten line can desynchronise. Check both directions.

- **Mobs:** every folder under `src/mobs/<category>/` is declared in the matching `pub mod`
  block of `mod mobs`, **and** appears exactly once in `mob_kinds()`. Flag the two failure
  modes separately: declared but absent from `mob_kinds()` (invisible to `/abyss summon`),
  and present in `mob_kinds()` with no file (a compile error).
- Each mob's folder must match its own `category()`, since nothing enforces it at runtime.
- **Ids:** no duplicate `id()` within the mobs, none within the items, and every id fits in
  `i8` (`mark` silently writes nothing above `127`).
- **Items:** every module in `mod items` has a spelling accepted by the `AbyssItem` executor in
  `src/commands/abyss.rs`. There is no `item_kinds()` catalogue, so a kind with no arm there is
  dead code rather than a lookup miss.
- Flag an id that a deleted kind used being reused — ids are permanent and never reclaimed.

### 6. Cross-cutting identifiers

- Permission nodes and world names derive from `PLUGIN_ID`; flag any hand-written
  `mine_in_abyss` string or a `PLUGIN_ID` that is not the crate name flattened.

## How To Report

| Check              | Item            | Status | Issue                           |
| ------------------ | --------------- | ------ | ------------------------------- |
| Declared in lib.rs | `events/foo.rs` | ❌     | File exists, no `pub mod`       |
| Event registered   | `Foo`           | ❌     | Handler defined, not registered |
| Layer complete     | `Layer::Fourth` | ✅     |                                 |
| Permission node    | `abyss`         | ✅     | Built from `PLUGIN_ID`          |

End with a summary (N modules, N handlers, N commands, N warnings, N errors) and ask whether
the user wants the mismatches fixed.
