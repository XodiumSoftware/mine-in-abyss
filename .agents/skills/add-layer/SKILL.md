---
name: add-layer
description: Adds a new layer to mine-in-abyss by adding a Section variant, its name, palette and world entry.
---

# Add a Layer

A layer **is** a `Section` variant. There is no per-layer struct or trait — identifiers,
display names, palettes and world names all live on `Section` in `src/section.rs`.

**Discover, don't assume.** Read `src/section.rs` and `src/lib.rs` before writing
anything. An earlier version of this skill described a `Layer` trait in `src/layers/`
plus a config system (`src/config.rs`, `enabled()`, `cmds()`/`perms()`); **none of that
exists**.

## Steps

1. **`src/section.rs`** — add the variant to the `Section` enum **in depth order**
   (`next`/`prev`/`index`/`curse` all derive from variant order), then fill in its arm in:
    - `Section::ALL`
    - `Section::name()`
    - `Section::color()`

2. **`src/lib.rs`** — `LAYERS` is `Section::ALL`, so there is usually nothing to change.

3. **Behaviour**, if the layer needs any: implement `EventHandler<T>` in a module under
   `src/events/` and register it in `MineInAbyss::register_events`. Prefer one handler
   that covers all layers (as `LayerLeave` does) over per-layer registration.

## Constraints

- **No `//` line comments.** Explain intent with `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- Names and colours belong to `Section::name()` / `Section::color()`, never to a second
  place that could drift from the enum.
- `Section::VOID_Y` / `Section::MAX_Y` are shared by every layer: each layer is its own
  full-height world. Do not add per-layer heights.
- Each layer world is a **hand-built map**. Do not assume ground at any coordinate, and
  never regenerate layer world data on startup.

## Verification

```bash
cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings
cargo build --target wasm32-wasip2
cargo doc --no-deps --target wasm32-wasip2
```

Then update `AGENTS.md` (§ Layer System, § Adding a New Layer) if the tables changed.

After finishing, summarize the files changed and ask the user whether to commit.
