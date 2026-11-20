# mine-in-abyss — Agent Context

## Project at a Glance

- **Type:** Pumpkin Minecraft plugin (WASM), Rust 2024, target `wasm32-wasip3`
- **Output:** `mine_in_abyss.wasm` (named after the `[lib]` target, so underscores)
- **Purpose:** Recreates *Made in Abyss*. Each layer is a separate Minecraft world.
- **Worlds:** every layer is a **hand-built map** — authored world data, not generated
  terrain. Never write code that resets, regenerates or overwrites layer world data.

## Documentation Rule — read this first

**Do not update the docs as part of a code change.** The code and its rustdoc are the source
of truth; `AGENTS.md` records only what a reader cannot infer from the source. After a change,
leave the docs alone unless the user explicitly asks for them.

- **Do not** rewrite `AGENTS.md`, `GUIDE.md`, `ARCHITECTURE.md` or module docs to mirror a
  change, and do not add tables that repeat what the source already states.
- **Do** write a short `//!` or `///` note when the *reason* is not visible in the code — a
  non-obvious constraint, a workaround, a number that is a guess rather than data.
- If a doc and the code disagree, trust the code and fix the doc *only* where it would mislead.

## Quick Commands

Prefer `just`. `build` is release-only, because debug builds fail to load in the Pumpkin
runtime (a wasmtime locals limit in `handle-event`).

| Task | Command |
| ---- | ------- |
| Everything CI runs | `just validate` |
| Lint, pedantic, warnings-as-errors | `just lint` |
| Format / check | `just fmt` / `just fmt-check` |
| Build the release WASM | `just build` |
| Build and deploy to `.server/plugins/` | `just build-copy` |
| Generate rustdoc | `just doc` |

The repo pins `wasm32-wasip3`. If that target is not installed, run raw `cargo` with
`--target wasm32-wasip2` instead and say so.

## Conventions

- No `//` line comments. Intent goes in `//!`, `///`, or the names.
- No `mod.rs`: folder modules are declared inline in `lib.rs`.
- No config system; no `serde`/`toml`.
- `unsafe_code` is forbidden. All clippy lints warn; CI adds `-W clippy::pedantic -D warnings`.
- Permission nodes are `{PLUGIN_ID}:command.{name}`, derived through `Command::permission_node`.
- No `MiniMessage` — the API has no parser. Build `TextComponent`s directly.
- `PLUGIN_ID` is `"mine_in_abyss"`, because the crate name's hyphen is not a valid namespace.

## Registration

`on_load` runs, in order: `layers`, `mobs`, `events`, `commands`, `worlds`. Each iterates a
single array and logs its length, so a logged count cannot drift from what was registered.

- **Events** — add the file, its submodule in `lib.rs`, its `impl Event`, and one entry in
  `MineInAbyss::events`.
- **Commands** — same shape with `impl Command` and `MineInAbyss::commands`.
- **Layers** — a `Layer` variant, its arms in `name`/`color`/`ALL`. `LAYERS` is `Layer::ALL`.

## Invariants worth knowing

These are the rules a change is most likely to break silently.

**Layers.** `Layer` (in `layer.rs`) owns every per-layer fact: id, world name, name, palette,
`next`/`prev`, `index`, `curse`, `game_rules`, `entry_y`/`exit_y`. There is no layer trait and
no per-layer module. Per-layer **data** is a `const fn` with a `match` on the variant;
per-layer **behaviour** belongs in the event handler that matches on the layer.

**Transitions.** `LayerLeave` compares a move against `Layer::MIN_Y` (`-64`, the world floor)
and `Layer::MAX_Y` (`320`). Below the floor → next layer at `entry_y()`; above the ceiling →
previous layer at `exit_y()`. X/Z are preserved. **Transitions are one-shot**: `utils::
begin_transition` latches a player and `LayerJoin` releases it on the world change — not
immediately after the teleport, or the following moves re-trigger. The latch is deliberately
not persistent.

**Teleports are not climbs.** `PlayerTeleportEvent` carries no world, so `LayerTeleport`
records the destination Y and `LayerLeave` consumes it once.

**Fall damage.** A world change resets `get_fall_distance`, so a descent carries its distance
one layer via `utils::carry_fall`. `FallDamage` adds it until the player passes `MIN_Y`. Damage
is never applied directly, so water and feather falling keep working. Only one layer is
carried. A teleport or respawn clears the carry.

**Curses and spawns.** Ascending adds to a persistent strain byte; reaching `Orth` or
respawning cures it; strain 6 is fatal. Beds cannot set a spawn point (`BedSpawn` cancels
unless `forced`, which is how `/spawnpoint` works). Death returns the player to `Orth`'s spawn,
and `LayerRespawn` must supply the world itself.

**Creative, spectator and dead players are skipped** by every transition handler.

**Custom entities are vanilla entities wearing an identity.** Entity types and `Item` are
closed enums, so nothing can be registered. The identity is a signed byte in the entity's or
stack's own persistent data under `PLUGIN_ID`. A kind is a **plain constructor**, not a trait
impl — `EntityBuilder::of(id, base).name(..).apply(..).build(world, pos)`.

- The byte space is shared between mobs, traders and items. **Ids are permanent once shipped**;
  a new kind takes the next free number.
- Mobs hold `1`–`65`, traders `66`–`101`, items `1`–`124`. Check the tables in the source
  before claiming a number — the catalogues are `mob_kinds()` in `lib.rs` and the item folders.
- Attributes go through `set_attribute_base`, and raising `MaxHealth` means raising
  `set_health` to match.

**Mob AI.** `mob_kinds()` is the only spawn table: `CreatureSpawn` cancels a natural vanilla
spawn in a layer world and spawns a kind whose `LAYERS` lists that layer. `Sixth` and `Seventh`
list no kinds, so they contain no mobs at all. Entities cannot change layers, so a mob lives
and dies where it spawned.

- Goals come from `BuiltinAiGoal` via `entity.as_mob()`. A vanilla base type arrives with its
  own goals, so a kind that must not attack has to `clear_ai_goals()` first.
- **Custom goals are unreachable**: the API declares `ai::AiGoal` and the host exports
  `handle-ai-goal-*`, but the registry behind `add_custom_ai_goal` is `pub(crate)`. Use
  built-ins until upstream exposes a register call.
- Pathfinding can be tuned with `set_pathfinding_malus`.

**Trades cannot be authored yet** — `villager-data` is only profession, level and experience,
and there is no setter for offers. Do not invent a trade table elsewhere; the wiki
(`wiki/src/traders/index.md`) is the record.

## Known constraints

- No entity model, texture or hitbox support; equipment is the only visual lever.
- Entity attributes cannot change a hitbox, and status effects only apply to players.
- Only players change layers: there is no entity move or world-change event.
- `set_custom_data` takes an opaque `NbtTree` with no constructor, so use
  `set_byte`/`get_byte` for anything the plugin stores.
- Difficulty is server-wide and has no setter; express peaceful behaviour as game rules.
- `CreatureSpawn`'s type string is a **numeric registry id**, not a name, and can renumber
  after an API upgrade.

## Testing

The project has no unit tests. Integration testing is a release build loaded into a Pumpkin
server. If you cannot run validation, say so rather than claiming it passed.

## Skills

Reusable procedures live in `.agents/skills/`, one `SKILL.md` per task. Read the one whose
`description` matches the job. None of them authorise rewriting the docs — see
[Documentation Rule](#documentation-rule--read-this-first).

## Workflow

- Number tasks in their names (e.g. "1. Add MOTD module").
- After each task, ask whether to commit. When all tasks are done, ask whether to push.
- Never commit or push unprompted.
