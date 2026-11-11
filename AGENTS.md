# mine-in-abyss — Claude Code Context

## Project at a Glance

- **Name:** mine-in-abyss
- **Type:** Pumpkin Minecraft plugin (WASM)
- **Purpose:** The recreation of 'Made In Abyss' into Minecraft.
- **MC Version:** Latest Pumpkin (tracks nightly)
- **Language:** Rust (Edition 2024)
- **Build Tool:** Cargo
- **Output:** `mine_in_abyss.wasm` (WASI Preview 2 — named after the `[lib]` target, so
  underscores, not the package name)
- **Target:** `wasm32-wasip2`
- **Worlds:** One Minecraft world per layer, each a **custom hand-built map** authored
  for this project — not generated terrain. See [Each layer is a hand-built map](#each-layer-is-a-hand-built-map).

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

**Always prefer `just` over raw `cargo`.** The recipes are cross-platform and encode things the
bare commands miss — most importantly that `build` is **release** only, because debug builds
fail to load in the Pumpkin runtime (a wasmtime locals limit in the wit-bindgen `handle-event`
export). Run `just` with no arguments to list every recipe.

| Task                                | Command           |
| ----------------------------------- | ----------------- |
| Everything CI runs                  | `just validate`   |
| Lint, pedantic, warnings-as-errors  | `just lint`       |
| Check formatting                    | `just fmt-check`  |
| Format                              | `just fmt`        |
| Build the release WASM              | `just build`      |
| Build, deploy to `.server/plugins/` | `just build-copy` |
| Generate rustdoc                    | `just doc`        |

```bash
# Validate exactly as CI does: lint + format check + release build
just validate

# Build, and copy the plugin to .server/plugins/
just build-copy
```

Use raw `cargo` only for something `just` genuinely does not cover — a `cargo check`, a
`cargo add`, or a one-off `cargo doc --document-private-items`. If a `just` recipe fails and you
need a variation, say which command you ran and why.

## Architecture Overview

### Entry Point

**`MineInAbyss`** — implements `Plugin` from `pumpkin_plugin_api`:

1. **Registration**: Via `register_plugin!(MineInAbyss)` macro
2. **`on_load`**:
    - Registers the event handlers (see [Events](#events))
    - Creates one world per layer via `Worlds::setup`
3. **`on_unload`**: Logs farewell message

### Layer System

The Abyss is a vertical world, and each layer **is** a [`Layer`] variant. There is no
separate layer trait: identifiers, names, palettes and world names all live on `Layer`,
so a layer cannot be described in two places that drift apart.

`Layer` (`src/layer.rs`) names the strata from the surface down:

| Variant   | `id()`    | World                   | `name()`                      |
| --------- | --------- | ----------------------- | ----------------------------- |
| `First`   | `first`   | `mine_in_abyss_first`   | The Top of the Abyss          |
| `Second`  | `second`  | `mine_in_abyss_second`  | The Forest of Temptation      |
| `Third`   | `third`   | `mine_in_abyss_third`   | The Great Fault               |
| `Fourth`  | `fourth`  | `mine_in_abyss_fourth`  | The Goblets of Giants         |
| `Fifth`   | `fifth`   | `mine_in_abyss_fifth`   | The Sea of Corpses            |
| `Sixth`   | `sixth`   | `mine_in_abyss_sixth`   | The Capital of the Unreturned |
| `Seventh` | `seventh` | `mine_in_abyss_seventh` | The Final Maelstrom           |

It derives `EnumIter` + `IntoStaticStr` (with `serialize_all = "lowercase"`), so
`id()` is just `self.into()` — variant names are never hand-written twice. Built on
that:

- `world()` — `format!("{PLUGIN_ID}_{}", self.id())`
- `name()` / `color()` — display title and the two gradient stops
- `next()` / `prev()` — neighbouring strata, `None` at the ends
- `index()` / `number()` — depth index, `0`/`1` at the surface
- `curse()` — Curse of the Abyss severity, derived from `index()`
- `game_rules()` — per-world game rules, applied on world setup
- `entry_y()` / `exit_y()` — the shared spawn heights (see below)

`Layer::ALL` is the source of truth for which layers exist, and `LAYERS: [Layer; 7]`
in `src/lib.rs` is `Layer::ALL`. Registration, world creation and world-name lookup
(`layer_for_world`) all read from it.

### Events

Handlers live in `src/events/`, declared inline in the `mod events` block in `lib.rs`:

| Handler         | Event                     | Behaviour                                                                |
| --------------- | ------------------------- | ------------------------------------------------------------------------ |
| `LayerJoin`     | `PlayerChangedWorldEvent` | Shows the entered layer's gradient title; cures the curse on the surface |
| `LayerLeave`    | `PlayerMoveEvent`         | Void/ceiling detection, teleport, curse                                  |
| `FallDamage`    | `PlayerMoveEvent`         | Adds a carried descent to the fall distance in the layer arrived in      |
| `Monsters`      | `EntitySpawnEvent`        | Scales a spawned monster's stats by the layer's depth                    |
| `BedSpawn`      | `PlayerSpawnChangeEvent`  | Refuses a spawn point a player sets, but keeps `/spawnpoint`             |
| `LayerRespawn`  | `PlayerRespawnEvent`      | Clears the curse strain, then returns the player to `First`'s spawn      |
| `LayerTeleport` | `PlayerTeleportEvent`     | Records where a teleport put the player, so it is not read as a climb    |

`LayerLeave` compares against the bounds shared by every layer
(`Layer::MIN_Y = -64` — the world floor, which is also where a player counts as having
left the layer — and `Layer::MAX_Y = 320`, just above the build limit): below the floor
→ next layer down at `entry_y()` (`MAX_Y - 2.0`); above the ceiling → previous layer up at
`exit_y()` (`MIN_Y + 1.0`). Both keep X/Z. Falling out of `Seventh` (no `next()`) is
left alone — vanilla void damage kills the player. Ascending calls `curse::apply`, which
adds to the player's accumulated strain.

**Creative and spectator players are skipped**, as are **dead** players: a player in
`GameMode::Creative`, `GameMode::Spectator`, or with `is_dead()` true returns from the
handler untouched, so they are never teleported or cursed.

**Thresholds are plain constants.** `MIN_Y` and `MAX_Y` are read on every move, so they
are `f64` constants rather than world queries. The world floor is `-64` for every layer,
which is the Overworld dimension height every layer world uses.

**Teleports are not climbs.** `PlayerTeleportEvent` has no world field, so `LayerTeleport`
records only the destination Y via `utils::note_teleport`. `LayerLeave` then consumes that
record once: if the next move is within `TELEPORT_TOLERANCE` of it, the move is ignored
rather than treated as an ascent. An operator teleporting overhead is therefore not
cursed, while a real climb afterwards still is.

**Transitions are one-shot.** `PlayerMoveEvent` fires on every movement, so without a
latch a player inside the void or ceiling band would be teleported (and cursed) on every
tick. `utils::{begin_transition, end_transition}` keep a thread-local set of player ids
currently transitioning; the second call returns `false` and the handler bails.

The latch is taken in `LayerLeave` and released in `LayerJoin` on
`PlayerChangedWorldEvent` — not immediately after the teleport — so the moves and world
change that follow cannot re-trigger the transition. A transition that starts but cannot
finish (missing world, or a terminal layer) releases the latch itself. It is deliberately
**not** persistent, so a stale entry can never survive a restart.

A successful transition also returns the move event with `cancelled: true`, so the
server does not apply the original `to_position` — which would move the player back into
the band of the world they just left.

**Leaving the Abyss at either end tells the player.** Rising above `First` or falling
below `Seventh` has nowhere to go, so `LayerLeave` sends a grey action bar message instead of
silently doing nothing.

**A descent carries its fall distance.** Changing world resets `get_fall_distance`, and a
layer is only 384 blocks tall, so per-world falls would never reach a damaging distance.
`LayerLeave::down` measures the distance before the teleport and
`utils::carry_fall` keeps it; `FallDamage` (`src/events/fall_damage.rs`) then adds it to
the fall in the layer arrived in on every move until the player passes `Layer::MIN_Y`,
where `utils::take_fall` drops it and vanilla damages on impact. Damage is never applied
directly, so water, hay bales and feather falling keep working. A teleport or respawn
clears the carry. **Only one layer is carried**: a player who falls from `First` without
landing is hurt by the last segment, not the whole drop.

**Monsters are exponentially harder by layer.** `Layer::monster_scale()` is `2^index`, so
`First` is `1x` and `Seventh` is `64x`. `Monsters` (`src/events/monsters.rs`) applies it on
`EntitySpawnEvent` to every monster in `MONSTERS`, multiplying the **base** `MaxHealth`,
`AttackDamage` and `FollowRange` so re-applying never compounds. `EntitySpawnEvent`, not
`CreatureSpawnEvent`, because natural spawns, spawners and `/summon` all pass through it.
A monster that follows a descent is rescaled by `Worlds::descend_monsters`.

**The spawn event's type string is a numeric id.** Pumpkin sends
`entity_type.id.to_string()` — the registry index (`zombie` is `"154"`), not a name. The
`monster_id` table maps both the id and the `resource_name`, and it is the only place
those numbers live. They are tied to Pumpkin's registry order and must be re-verified
after an API upgrade: if it renumbers, scaling silently stops matching.

**Beds cannot set a spawn point.** `BedSpawn` cancels `PlayerSpawnChangeEvent` unless it is
`forced` or is clearing a point (`new_spawn: None`). `forced` is how the host marks
`/spawnpoint`, and beds and respawn anchors both pass `forced: false`, so one flag
separates an operator override from a player shortcut — do not drop it, or `/spawnpoint`
stops working. Cancelling is sufficient: the host returns `false` from
`set_respawn_point` and sends no "Spawn point set" message.

**Dying returns the player to `First`'s world spawn.** `PlayerRespawnEvent` is **not
cancellable** and fires after the host resolved a destination, so `LayerRespawn` respawns
the player and then teleports them to `First`. It must supply the world: without a spawn
point the host falls back to the `OVERWORLD` dimension's spawn, which is not a layer world,
and `PlayerSpawnLocationEvent` can move a position but not a world.

**Only players change layers.** There is no entity move event or entity world-change event
in the plugin API, so an item or mob thrown into the void does not descend — it falls to
the world floor and takes vanilla void damage. This is deferred deliberately, **not** a
missing feature to fix with a scheduler poll over `World::get_entities()`, which would scan
every entity in all seven worlds on a repeating task. `ARCHITECTURE.md` records what an
implementation would need, and why `EntityMoveEvent` upstream is the trigger to revisit it.

Titles are built as `TextComponent`s directly — the plugin API has no `MiniMessage`
parser, so **never emit `MiniMessage` strings**. `color_named` and friends are
consuming builders: reassign the result (`let t = TextComponent::text(..).color_named(..)`)

### Commands

Commands live in `src/commands/<name>.rs` and are declared inline in the `mod commands`
block in `lib.rs`. Each module exposes `register(context)`, which is called from
`on_load`.

| Command  | Aliases | Permission                    | Behaviour                                                    |
| -------- | ------- | ----------------------------- | ------------------------------------------------------------ |
| `/abyss` | `/mia`  | `mine_in_abyss:command.abyss` | Moves a player into a layer without the curse (ops level 3+) |

`/abyss <layer> [player]` accepts `first` … `seventh` or `1` … `7`, case-insensitive.
A player running it is always the one moved; `[player]` is only consulted from console.
It teleports straight into the layer, so no curse strain is added — unlike a real ascent.

Tab-completion for `<layer>` is **disabled**: attaching it makes the Pumpkin host panic on
the first suggestion request (`block_in_place` from the simulation thread, which has no
Tokio reactor). The `LayerSuggestions` handler is kept in `src/commands/abyss.rs` behind
`cfg(any())` for when that is fixed.

Registration goes through the raw `Context::register_command(command, node)` plus
`Context::register_permission(&Permission)`. Permission nodes are built from
`PLUGIN_ID` (`abyss::permission_node()`), never hand-formatted. Handler structs
implement `pumpkin_plugin_api::commands::CommandHandler` and return `Ok(1)` on success.

### Curse of the Abyss

`src/events/curse.rs` stores a `curse` strain byte on the player via
`PersistentDataHolder` (namespaced NBT, persisted to disk), so it survives relogs.

- `strain(player)` / `cure(player)` read and clear it
- `apply(player, from)` adds `from.curse()` to the strain, saturating at `MAX_STRAIN`
  (6), then inflicts the symptoms for the **total**
- Ascending several layers in a row therefore compounds: the curse is cumulative
- Reaching `First`, or respawning, calls `cure`
- Strain 6 is fatal — `inflict` kills rather than applying effects

### Worlds

**`Worlds`** (`src/worlds.rs`) — creates one Minecraft world per layer on load.

- `Worlds::setup(context, layers)` runs from `on_load`
- World names come from `Layer::world()`, so they stay stable across renames
- `create_world` is create-or-load: an existing world is reused, never reset
- `World` is a non-cloneable WIT resource, so only the set of ready world names is
  cached in a `thread_local`; callers re-resolve the handle from the server
- A failure for one layer is logged and does not stop the others

#### Per-layer content

Per-layer **data** belongs on `Layer` as a `const fn` with a `match`, the way
`name()`, `color()` and `game_rules()` do; per-layer **behaviour** belongs in one
event handler that matches on the layer (as `LayerLeave` does). Terrain, structures and
points of interest are neither — they are world data.

`Layer::game_rules()` is the worked example: `First` disables `spawn-monsters`, the
rest return an empty slice. `Worlds::setup` applies the rules on every load, so a world
folder saved with different rules cannot override them.

Minecraft's **difficulty** is a server-wide setting and the plugin API exposes no setter
for it (`Server::get_difficulty` only reads), so peaceful behaviour is expressed as
per-world game rules instead.

### Time of day

Every layer world keeps its own clock, so left alone they drift apart and a descending
player would watch the sun jump. `src/time_sync.rs` copies `First`'s clock to the other six
worlds on a one-minute repeating task, which runs once immediately on load.

`First` is the source because it is the surface, the layer an operator is most likely to
change the time in, and the start of every descent. Change the time there and the rest of
the Abyss follows within a minute.

The copied value is the full clock, not just the time within a day, so the **day number**
follows as well — `/time set 24000` in `First` puts the other layers on day 1 too. A
world's **age** is a separate counter with no setter in the plugin API, so it cannot be
shared; nothing in vanilla depends on age and time of day agreeing.

#### Each layer is a hand-built map

Every layer world is a **custom map made specifically for this project** — an authored
region with real terrain, structures and points of interest, not a generated world. This
is the central assumption behind the whole design, so keep it in mind whenever you touch
transitions or placement:

- `create_world` only takes a name and dimension — there is no generator, seed or
  level-type argument. The map **is** the world folder on disk, so shipping a layer means
  shipping its world data.
- Worlds are generated **empty**, not left to the default generator: `Worlds::setup`
  registers the [`Void`](src/generator.rs) `ChunkGenerator` and attaches it with
  `World::set_chunk_generator` before the world is used. Generation is the only way to
  pick what a layer starts as, since `create_world` offers no level-type argument.
- A world that already has chunks keeps them, so an authored map is safe across restarts
  and the generator only decides what a not-yet-existing world looks like. Never write
  code that resets, regenerates or overwrites layer world data on startup.
- Because the terrain is authored, **do not assume there is ground at a given X/Z**. The
  transition code preserves X/Z and picks a Y from `Layer::entry_y()` / `exit_y()`;
  whether that lands on solid ground depends entirely on the map at those coordinates.
- If a layer needs a guaranteed arrival point, that must be map data (a spawn point or a
  marker) or an explicit safe-position lookup — never a hardcoded coordinate. An empty
  layer has no ground at all, so nothing keeps a player from falling straight through.
- Void and ceiling thresholds (`Layer::MIN_Y` / `Layer::MAX_Y`) are the world floor
  and the build limit, which is why every layer can be a full-height world without
  interfering with its neighbours.

### Configuration

**None.** The TOML config system was removed; there is no `config.rs`, no
`config.toml`, and layers do not gate on config.

### Active Layers

All seven layers exist as `Layer` variants (identifier, world, name, gradient colour)
with no gameplay content yet. See the table under [Layer System](#layer-system).

There are no per-layer modules or structs: everything a layer needs is on its `Layer`
variant, so adding a layer means adding a variant rather than a file.

`utils.rs` also holds `ordinal`, the English ordinal-suffix formatter used for the entry
subtitle.

### Documentation

| File              | Audience      | Contents                                                       |
| ----------------- | ------------- | -------------------------------------------------------------- |
| `ARCHITECTURE.md` | developers    | Mermaid diagrams of the layer model, transitions, latch, curse |
| `GUIDE.md`        | server owners | Install, build, player-facing behaviour, troubleshooting       |
| `AGENTS.md`       | agents        | This file: conventions and invariants                          |

`ARCHITECTURE.md` is the place to update whenever a transition, the latch or the curse
changes shape — the diagrams are the fastest way to see the design at a glance.

### Project Structure

```
src/
├── lib.rs              # `PLUGIN_ID`, `LAYERS`, `layer_for_world`, `Plugin` impl
├── layer.rs            # `Layer` enum + id/world/next/prev/index/curse,
│                       # plus shared `MIN_Y`/`MAX_Y` and entry/exit heights
├── utils.rs            # Shared helpers (`ignores_transitions`, `layer_of`, `tell`, `world_for`)
├── generator.rs        # `Void` chunk generator — layer worlds generate empty
├── time_sync.rs        # Keeps every layer world on `First`'s clock
├── worlds.rs           # `Worlds` — one void world per layer
├── commands/
│   └── abyss.rs        # `/abyss` — ops teleport into a layer, curse-free
├── events/
│   ├── curse.rs        # Curse of the Abyss: strain storage + severity ladder
│   ├── bed_spawn.rs    # `BedSpawn` — refuses beds setting a spawn point
│   ├── fall_damage.rs  # `FallDamage` — carries a descent's fall distance one layer
│   ├── layer_join.rs   # `LayerJoin` — entry title, cure on the surface
│   ├── layer_leave.rs  # `LayerLeave` — void/ceiling teleport + curse
│   ├── monsters.rs     # `Monsters` — mob stats scale exponentially with depth
│   ├── layer_respawn.rs# `LayerRespawn` — respawns at First's spawn
│   └── layer_teleport.rs # `LayerTeleport` — teleports are not climbs
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- **No `//` line comments.** Explain intent in rustdoc (`///`, `//!`) and in identifier
  and function names instead. Only `///` and `//!` comments belong in `src/`.
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Config structs: `Debug`, `Clone`, `Default`, `Serialize`, `Deserialize`
- Config fields documented with `///` comments
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `mine_in_abyss:command.example`)
- No config system
- Event handlers return `EventData<T>` (may modify event)
- Release profile: LTO + strip for minimal WASM size
- **Before checking API availability, run `cargo update` to fetch the latest `pumpkin-plugin-api` revision.** The API is git-tracked and may have added new events or types since the last local checkout.

### Code Style

**Order of items in layers:**

1. **`//!`** — Module-level documentation
2. **`use`** — imports (external, crate, std)
3. **`const`** — module constants (e.g., `PLUGIN_ID`)
4. **`struct`**/**`enum`** definitions (documented fields)
5. **`impl Layer`** — layer trait implementation
6. **`impl EventHandler<T>`** — event handler implementations
7. **`impl OtherTrait`** — other trait implementations

**Documentation:**

- Module-level docs with `//!` describing the layer's purpose and its Y range
- All `pub` items have `///` rustdoc comments

**Error Handling:**

- Use `tracing::{info, warn, error}!` for logging, not `println!`
- Event handler failures use `.expect()` for critical registrations

## Testing

- The project intentionally has **no unit tests**
- Integration testing: build WASM and load in Pumpkin server

## Important Notes

- A crate name with a hyphen (`mine-in-abyss`) can't be a Minecraft namespace, so `PLUGIN_ID` is the literal `"mine_in_abyss"`
- There is no config file; the plugin has no persisted settings
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
    - **Run `just doc`** to verify

**Rule of thumb:** If a code change would confuse someone reading the docs, update the docs.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:

- **rust.yml** — Lints, builds the WASM plugin on push/PR, uploads the artifact and publishes the nightly release
- **enforce_pr_title.yml** — Validates PR titles follow conventional commits

## Adding a New Layer

To add a new layer, follow these steps:

1. In `src/layer.rs`: add a variant to the `Layer` enum (keep it in depth order —
   `next`/`prev`/`index`/`curse` all derive from the variant order)
2. Fill in its arm in `Layer::name()`, `Layer::color()` and `Layer::ALL`
3. In `src/lib.rs`: `LAYERS` is `Layer::ALL`, so nothing to change unless the layer
   needs its own behaviour
4. If the layer needs behaviour of its own, implement `EventHandler<T>` in
   `src/events/` and register it in `MineInAbyss::register_events` — prefer one handler
   for all layers (as `LayerLeave` does) over per-layer registration
5. Run `just lint` (and `just validate` before a PR)

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
