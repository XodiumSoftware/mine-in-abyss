---
name: bootstrap-audit
description: Audits mine-in-abyss registration wiring so every feature file is declared, has a config toggle, and is actually registered.
---

# Bootstrap Audit

Use this skill when the user asks to verify plugin wiring, when features are missing at
runtime, or after adding/renaming several of them. Features can easily end up declared but
never registered.

This is a **read-only** audit. Report findings in a table; only fix things if the user asks.

**Derive the checks from the code, don't hardcode them.** Start by reading `src/lib.rs` and
the registration trait, then generate the file inventory with searches (e.g. glob `src/**/*.rs`
and exclude trait/config/mod files) rather than relying on a fixed folder list.

## What To Check

### 1. Declaration vs. file existence

- Every feature file found under `src/` (excluding the trait, its config, and shared helpers)
  is declared in the `mod` tree in `src/lib.rs`, including any intermediate folder `mod.rs`.
- The `mod` tree matches the on-disk layout — flag stale or missing `pub mod` lines.

### 2. Registration vs. declaration

- Every declared feature that the trait requires registration for appears in the registration
  list in `src/lib.rs` (find it inside the `register_*` function called from `on_load`).
- Flag declared-but-unregistered features, and registered-but-nonexistent imports.

### 3. Config toggle coverage

- Every feature has a field in the config struct that owns its settings, a matching re-export
  if the file re-exports neighbours, and an `enabled()` that reads its own field.
- Flag orphaned config fields (no matching feature) and features with no toggle.
- Flag any group toggle in `src/config.rs` / `config.toml` that gates a group with no members.

### 4. Group/position consistency

- If features declare a position (stratum, depth, category) via a method or config, check it
  matches the folder they live in. Flag mismatches.

### 5. Command/permission pairing

- For features with commands: `cmds()` and `perms()` are paired by index in the trait's
  `register()` — flag mismatched lengths or orderings.

## How To Report

| Check | Feature | Status | Issue |
|---|---|---|---|
| Declared in lib.rs | `Example` | ❌ | File exists, no `pub mod` |
| Config toggle | `example` | ✅ | |
| Registered | `Example` | ❌ | Missing from registration list |

End with a summary (N features found, N warnings, N errors) and ask whether the user wants
the mismatches fixed.
