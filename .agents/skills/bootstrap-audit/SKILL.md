---
name: bootstrap-audit
description: Audits mine-in-abyss registration wiring so every module file is declared in lib.rs, has a config toggle, and is actually registered.
---

# Bootstrap Audit

Use this skill when the user asks to verify the plugin wiring, when modules appear to be missing at runtime, or after adding/renaming several modules. Modules can easily end up declared but never registered, so audit regularly.

## What To Check

Run this read-only audit and report a table of findings. Only fix things if the user asks.

### 1. Module declaration vs. file existence

For each of these folders, list the `.rs` files (excluding the trait file) and check each has a matching `pub mod {name};` in `src/lib.rs` under the right `mod modules { ... }` branch:

- `src/modules/mechanics/{category}/*.rs` → `mod modules { mod mechanics { mod {category} { ... } } }` in `src/lib.rs`

### 2. Registration vs. declaration

Each declared mechanic must be registered in `src/lib.rs`:

- Mechanics → `MineInAbyss::register_mechanics` `mechanics: Vec<&dyn Mechanic>`

Flag any declared-but-unregistered module, and any registered-but-nonexistent import (stale wiring).

### 3. Config toggle coverage

- Mechanics: every mechanic must have a `{Name}Config` field in `MechanicsConfig` (`src/modules/mechanics/mechanic.rs`), a matching `pub use` at the top of that file, and its `enabled()` must read `cm.mechanics.{snake}.enabled`.
- Cross-check that no *extra* config fields exist without a corresponding registered module (orphaned toggles), and no registered module is missing its toggle.

### 4. Command/permission pairing

For mechanics with commands: `cmds()` and `perms()` are paired by index in `Mechanic::register`. Flag mechanics where the vec lengths or intended orderings look mismatched.

## How To Report

Present a table:

| Check | Module | Status | Issue |
|---|---|---|---|
| Declared in lib.rs | `RelicDetector` | ❌ | File exists, no `pub mod` |
| Config toggle | `relic_detector` | ✅ | |
| Registered | `CurseOfTheAbyss` | ❌ | Missing from `mechanics` vec |

End with a short summary: N modules found, N warnings, N errors. Ask whether the user wants the mismatches fixed before editing anything.
