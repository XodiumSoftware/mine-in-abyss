---
name: add-layer
description: Adds a new layer to mine-in-abyss by adding a Layer variant, its name, palette and world entry.
---

# Add a Layer

A layer **is** a `Layer` variant. There is no per-layer struct or trait — identifiers,
display names, palettes and world names all live on `Layer` in `src/layer.rs`.

**Discover, don't assume.** Read `src/layer.rs` and `src/lib.rs` before writing
anything. An earlier version of this skill described a `Layer` trait in `src/layers/`
plus a config system (`src/config.rs`, `enabled()`, `cmds()`/`perms()`); **none of that
exists**.

## Steps

1. **`src/layer.rs`** — add the variant to the `Layer` enum **in depth order**
   (`next`/`prev`/`index`/`curse` all derive from variant order), then fill in its arm in:
    - `Layer::ALL`
    - `Layer::name()`
    - `Layer::color()`
    - `Layer::game_rules()` (see below)

2. **`src/lib.rs`** — `LAYERS` is `Layer::ALL`, so there is usually nothing to change.

3. **Behaviour**, if the layer needs any: implement `EventHandler<T>` in a module under
   `src/events/`, implement `Event` beside it, and add one entry to the array in
   `MineInAbyss::events`. Prefer one handler that covers all layers (as `LayerLeave` does) over
   per-layer registration — see the `add-event` skill.

## Per-layer game rules

`Layer::game_rules()` returns the rules that layer's world runs with, applied by
`Worlds::setup` on **every** startup — so a world folder saved with different rules cannot
override them. An empty slice means vanilla defaults.

```rust
Self::Second => &[(GameRule::SpawnMonsters, GameRuleValue::Bool(false))],
```

- `GameRule` / `GameRuleValue` come from `pumpkin_plugin_api` (re-exported at the root).
- `GameRuleValue` is either `Int(i32)` or `Bool(bool)` — check `wit/game-rules.wit` for what a
  given rule expects.
- **Minecraft difficulty is server-wide** and the plugin API has no setter
  (`Server::get_difficulty` only reads). Express "peaceful" as game rules
  (`spawn-monsters`, `natural-health-regeneration`), not as a difficulty call.
- Spell out every variant in the `match` rather than using `_`, so adding a layer forces a
  conscious decision about its rules.

## Constraints

- **No `//` line comments.** Explain intent with `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- Names and colours belong to `Layer::name()` / `Layer::color()`, never to a second
  place that could drift from the enum.
- `Layer::MIN_Y` / `Layer::MAX_Y` are shared by every layer: each layer is its own
  full-height world. Do not add per-layer heights.
- Each layer world is a **hand-built map**. Do not assume ground at any coordinate, and
  never regenerate layer world data on startup.

## Verification

Run `just validate` and `just doc`. The release build is required: debug builds fail to load in
the Pumpkin runtime.

Then update `AGENTS.md` (§ Layer System, § Adding a New Layer) if the tables changed.

Note that a new layer also shifts several things sized to the current eight: `Layer::ALL`
and `LAYERS` in `lib.rs` are both `[Layer; 8]` and must change length together, and
`AGENTS.md` spells the count out in a few places (Layer System, Active Layers, Time of day).
Grep for "eight" and "seven" rather than trusting the tables alone.

**A ninth layer touches the curse.** `Layer::curse()` is the depth index, and `MAX_STRAIN` in
`src/events/curse.rs` is fixed at 6. Adding a layer below `Seventh` makes the deepest severity
8, so raise `MAX_STRAIN` and add the matching arm to `inflict` in the same change — see the
note in `ARCHITECTURE.md` and the `add-util` and `add-persistent-data` skills.

After finishing, summarize the files changed and ask the user whether to commit.
