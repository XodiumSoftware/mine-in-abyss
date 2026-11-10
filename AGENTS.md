# mine-in-abyss — Claude Code Context

## Project at a Glance

- **Name:** mine-in-abyss
- **Type:** Pumpkin Minecraft plugin (WASM)
- **Purpose:** The recreation of 'Made In Abyss' into Minecraft.
- **MC Version:** Latest Pumpkin (tracks nightly)
- **Language:** Rust (Edition 2024)
- **Build Tool:** Cargo
- **Output:** `mine-in-abyss.wasm` (WASI Preview 2)
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

| Task                               | Command           |
| ---------------------------------- | ----------------- |
| Everything CI runs                 | `just validate`   |
| Lint, pedantic, warnings-as-errors | `just lint`       |
| Check formatting                   | `just fmt-check`  |
| Format                             | `just fmt`        |
| Build the release WASM             | `just build`      |
| Build, fetch Pumpkin, deploy       | `just build-copy` |
| Generate rustdoc                   | `just doc`        |

```bash
# Validate exactly as CI does: lint + format check + release build
just validate

# Build, fetch the Pumpkin nightly server, and copy the plugin to .server/plugins/
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
| `LayerRespawn`  | `PlayerRespawnEvent`      | Clears the curse strain                                                  |
| `LayerTeleport` | `PlayerTeleportEvent`     | Records where a teleport put the player, so it is not read as a climb    |
| `LayerMobSpawn` | `CreatureSpawnEvent`      | Cancels spawns of mobs the layer does not allow                          |

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
below `Seventh` has nowhere to go, so `LayerLeave` sends a grey system message instead of
silently doing nothing.

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

`/abyss <layer> [player]` accepts `first` … `seventh` or `1` … `7`, case-insensitive,
and defaults the target to the sender. It teleports straight into the layer, so no
curse strain is added — unlike a real ascent.

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

#### Per-layer mob spawning

Game rules can only switch spawning off wholesale. To allow _some_ mobs and not others,
use `Layer::allowed_mobs()`, enforced by `LayerMobSpawn` on `CreatureSpawnEvent`:

| Return                        | Meaning                                            |
| ----------------------------- | -------------------------------------------------- |
| `None`                        | No restriction — every spawn is allowed            |
| `Some(&[])`                   | Nothing may spawn naturally                        |
| `Some(&["minecraft:pig", …])` | Only these may spawn; everything else is cancelled |

It is an **allowlist**: listing one mob forbids all others, so `Some(&[…])` is a
deliberate, per-layer decision rather than an additive one. Entity types are namespaced
entity ids (`minecraft:pig`); bare names are accepted too.

Current state: `First`–`Fourth` return `None` (unrestricted), `Fifth`–`Seventh` return
`Some(&[])` (nothing spawns). `First` stays `None` on purpose — its `spawn-monsters` game
rule already keeps hostiles out, so the allowlist would add nothing.

The handler checks `event.target_world`, so a spawn is judged against the layer it lands
in, and spawns in non-Abyss worlds are ignored. Spawns with `spawn_reason` `spawner` bypass
the allowlist, so an authored spawner always works.

#### Each layer is a hand-built map

Every layer world is a **custom map made specifically for this project** — an authored
region with real terrain, structures and points of interest, not a generated world and
not a plain void platform. This is the central assumption behind the whole design, so
keep it in mind whenever you touch transitions or placement:

- `create_world` only takes a name and dimension — there is no generator, seed or
  level-type argument. The map **is** the world folder on disk, so shipping a layer means
  shipping its world data.
- Create-or-load means a map that already exists on disk is reused untouched. Never write
  code that resets, regenerates or overwrites layer world data on startup.
- Because the terrain is authored, **do not assume there is ground at a given X/Z**. The
  transition code preserves X/Z and picks a Y from `Layer::entry_y()` / `exit_y()`;
  whether that lands on solid ground depends entirely on the map at those coordinates.
- If a layer needs a guaranteed arrival point, that must be map data (a spawn point or a
  marker) or an explicit safe-position lookup — never a hardcoded coordinate.
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

### Project Structure

```
src/
├── lib.rs              # `PLUGIN_ID`, `LAYERS`, `layer_for_world`, `Plugin` impl
├── layer.rs            # `Layer` enum + id/world/next/prev/index/curse,
│                       # plus shared `MIN_Y`/`MAX_Y` and entry/exit heights
├── utils.rs            # Shared helpers (`ignores_transitions`, `begin_transition`)
├── worlds.rs           # `Worlds` — one world per layer
├── commands/
│   └── abyss.rs        # `/abyss` — ops teleport into a layer, curse-free
├── events/
│   ├── curse.rs        # Curse of the Abyss: strain storage + severity ladder
│   ├── layer_join.rs   # `LayerJoin` — entry title, cure on the surface
│   ├── layer_leave.rs  # `LayerLeave` — void/ceiling teleport + curse
│   ├── layer_mob_spawn.rs # `LayerMobSpawn` — per-layer mob spawn allowlist
│   ├── layer_respawn.rs# `LayerRespawn` — clears the curse on respawn
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
