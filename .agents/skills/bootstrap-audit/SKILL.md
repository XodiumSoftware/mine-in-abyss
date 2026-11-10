---
name: bootstrap-audit
description: Audits mine-in-abyss registration wiring so every module is declared, every event handler and command is registered, and every layer is fully described.
---

# Bootstrap Audit

Use when the user asks to verify plugin wiring, when something is missing at runtime, or
after adding or renaming several modules. Things easily end up declared but never registered.

This is a **read-only** audit. Report findings in a table; only fix things if the user asks.

**Derive the checks from the code, don't hardcode them.** Read `src/lib.rs`, `src/layer.rs`
and `src/commands/` first, then build the inventory with searches (`find src -name '*.rs'`)
rather than relying on a remembered folder list.

There is **no config system and no feature/enabled flags**. An earlier version of this skill
audited `config.rs`, `enabled()` toggles and a `cmds()`/`perms()` trait; none of that exists.

## What To Check

### 1. Module declaration vs. file existence

- Every `.rs` file under `src/` is reachable from a `mod` declaration in `src/lib.rs`
  (directly, or through `mod commands` / `mod events`).
- The `mod` blocks match the on-disk layout: flag missing declarations and stale ones pointing
  at files that no longer exist.

### 2. Event handlers registered

- Every module in `mod events` that defines an `impl EventHandler<E>` appears in exactly one
  `register_event_handler::<E, _>` call inside `MineInAbyss::register_events`.
- Flag handler modules that are declared but never registered, and `register_event_handler`
  calls whose handler type no longer exists.
- `curse.rs` is a helper module, not a handler — it is correctly not registered. Any other
  non-handler module in `mod events` should be questioned.

### 3. Commands registered

- Every module in `mod commands` has a `pub fn register(context: &Context)` and is called from
  `MineInAbyss::on_load`.
- Each command registers both a command (`register_command`) and its permission
  (`register_permission`), and both use the same node built by the module's helper
  (`abyss::permission_node()`), not a hand-written string.

### 4. Layer completeness

- `Layer::ALL` lists every variant of the enum, in depth order.
- Every variant has an arm in `Layer::name()`, `Layer::color()` and `Layer::game_rules()`.
  The compiler enforces this for exhaustive `match`; flag any that used a `_` arm to dodge it.
- `LAYERS` in `src/lib.rs` is `Layer::ALL` — flag any other literal list of layers.

### 5. Cross-cutting identifiers

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
