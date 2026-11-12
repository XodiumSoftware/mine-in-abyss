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

| Category       | Technology                                                  | Purpose                     |
| -------------- | ----------------------------------------------------------- | --------------------------- |
| **Core API**   | [pumpkin-plugin-api](https://github.com/Pumpkin-MC/Pumpkin) | Minecraft server plugin API |
| **Language**   | Rust 2024                                                   | Systems language            |
| **Build Tool** | Cargo                                                       | Build automation            |
| **Logging**    | tracing                                                     | Structured logging          |
| **Docs**       | rustdoc (via `cargo doc`)                                   | API documentation           |

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
2. **`on_load`**, in order — four startup steps, one method each:
    - `MineInAbyss::layers` — logs how many layers exist
    - `MineInAbyss::mobs` — logs the mob kinds in the catalogue
    - `MineInAbyss::events` — registers every event handler (see [Events](#events))
    - `MineInAbyss::commands` — registers every command (see [Commands](#commands))
    - `MineInAbyss::worlds` — creates one world per layer via `Worlds::setup`, starts the
      clock sync task (`world::clock::schedule`), then logs that setup finished
3. **`on_unload`**: Logs farewell message

`layers`, `events` and `commands` each iterate a single array and log `array.len()`, so the
logged count cannot drift from what was registered.

`worlds` reports no count: `Worlds::setup` returns how many worlds came up, and that value
is deliberately discarded — a layer whose world is missing already logs a `warn!` of its
own. Its message therefore means _"setup finished and the sync task is running"_, not
"every world is up": the sync task runs once immediately, but after this line.

### Layer System

The Abyss is a vertical world, and each layer **is** a [`Layer`] variant. There is no
separate layer trait: identifiers, names, palettes and world names all live on `Layer`,
so a layer cannot be described in two places that drift apart.

`Layer` (`src/layer.rs`) names the strata from the surface down:

| Variant   | `id()`    | World                   | `name()`                      |
| --------- | --------- | ----------------------- | ----------------------------- |
| `Orth`    | `orth`    | `mine_in_abyss_orth`    | The City on the Edge          |
| `First`   | `first`   | `mine_in_abyss_first`   | Depths of the Abyss           |
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

`Layer::ALL` is the source of truth for which layers exist, and `LAYERS: [Layer; 8]`
in `src/lib.rs` is `Layer::ALL`. Registration, world creation and world-name lookup
(`layer_for_world`) all read from it.

### Events

Handlers live in `src/events/`, declared inline in the `mod events` block in `lib.rs`.
Each handler implements the [`Event`](src/events/event.rs) trait for itself, so the event
type it answers, its priority and whether it blocks stay in the file that defines the
handler. `src/events/event.rs` holds the trait; `MineInAbyss::events` in `lib.rs` calls
every `register` and logs the count. Registering a handler therefore means adding its file,
its submodule, its `impl Event`, and one entry in that array.

| Handler         | Event                     | Behaviour                                                              |
| --------------- | ------------------------- | ---------------------------------------------------------------------- |
| `LayerJoin`     | `PlayerChangedWorldEvent` | Shows the entered layer's gradient title; cures the curse in `Orth`    |
| `LayerLeave`    | `PlayerMoveEvent`         | Void/ceiling detection, teleport, curse                                |
| `FallDamage`    | `PlayerMoveEvent`         | Adds a carried descent to the fall distance in the layer arrived in    |
| `BedSpawn`      | `PlayerSpawnChangeEvent`  | Refuses a spawn point a player sets, but keeps `/spawnpoint`           |
| `LayerRespawn`  | `PlayerRespawnEvent`      | Clears the curse strain, then returns the player to `Orth`'s spawn     |
| `LayerTeleport` | `PlayerTeleportEvent`     | Records where a teleport put the player, so it is not read as a climb  |
| `CreatureSpawn` | `CreatureSpawnEvent`      | Replaces a naturally spawned vanilla mob with one of the layer's kinds |

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

**Leaving the Abyss at either end tells the player.** Rising above `Orth` or falling
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

**A descent leaves the monsters behind.** Entities cannot change layers and nothing moves them between worlds, so the mobs in the layer above stay there — what was chasing a player does not follow them down, and the layer they land in holds only its own inhabitants. **Monster stats do not scale with depth**: a mob is exactly what its type says, at every layer.

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

**Dying returns the player to `Orth`'s world spawn.** `PlayerRespawnEvent` is **not
cancellable** and fires after the host resolved a destination, so `LayerRespawn` respawns
the player and then teleports them to `Orth`. It must supply the world: without a spawn
point the host falls back to the `OVERWORLD` dimension's spawn, which is not a layer world,
and `PlayerSpawnLocationEvent` can move a position but not a world.

**Only players change layers.** There is no entity move event or entity world-change event
in the plugin API, so an item or mob thrown into the void does not descend — it falls to
the world floor and takes vanilla void damage. This is deferred deliberately, **not** a
missing feature to fix with a scheduler poll over `World::get_entities()`, which would scan
every entity in all eight worlds on a repeating task. `ARCHITECTURE.md` records what an
implementation would need, and why `EntityMoveEvent` upstream is the trigger to revisit it.

Titles are built as `TextComponent`s directly — the plugin API has no `MiniMessage`
parser, so **never emit `MiniMessage` strings**. `color_named` and friends are
consuming builders: reassign the result (`let t = TextComponent::text(..).color_named(..)`)

**Vanilla mobs never spawn in a layer world.** `CreatureSpawn` cancels a natural spawn in any
layer world and spawns a kind from [`mob_kinds`](src/lib.rs) whose `layers()` include that
layer, at the event's own position and world. The same `layers()` list decides what lives
where, and is also what `/abyss summon` enforced while it existed — there is no second spawn
table.

Only reasons in `NATURAL_REASONS` count as natural. A spawner, breeding, a spawn egg or a
command is left untouched, so this never removes a mob a player or operator asked for. The API
carries `spawn-reason` as a bare `string` with **no documented vocabulary**, so the spellings
are a best guess; an unrecognised one is logged at `debug` and not treated as natural. If
layers turn out to spawn nothing on a live server, check that log first — it distinguishes
"the host words it differently" from "no kind lives in this layer".

**The choice is positional, not random.** The kind is picked by mixing the spawn position's
bits into an index, so no RNG dependency is needed and the same spot always yields the same
kind. It is not weighted: every kind listing the layer is equally likely.

**Suppression has a visible consequence.** `Sixth` and `Seventh` list no kinds, so once this
handler is active those layers contain **no mobs at all** — vanilla spawns are cancelled and
nothing replaces them. That is the honest result of no kind being authored for them yet, but
it is a gameplay change, not an oversight. Every other layer has candidates (`Orth` 1, `First`
20, `Second` 21, `Third` 14, `Fourth` 18, `Fifth` 26).

**One place creates a mob now**, and nothing else does: this handler by rule. `/abyss summon`
used to be the second, for an operator asking, and is removed for now. Entities still cannot
descend between layers, so a mob spawned here lives and dies in the layer it was born in.

### Commands

Commands live in `src/commands/<name>.rs` and are declared inline in the `mod commands`
block in `lib.rs`. Each command implements the [`Command`](src/commands/command.rs) trait
for itself, so its tree, its permission node and the message a failed registration logs
stay in the file that defines the executor. `src/commands/command.rs` holds the trait;
`MineInAbyss::commands` in `lib.rs` calls every `register` and logs the count. Adding a
command therefore means adding its file, its submodule, its `impl Command`, and one entry
in that array.

| Command  | Aliases | Permission                    | Behaviour                                                    |
| -------- | ------- | ----------------------------- | ------------------------------------------------------------ |
| `/abyss` | `/mia`  | `mine_in_abyss:command.abyss` | Moves a player into a layer without the curse (ops level 3+) |

`/abyss <layer> [player]` accepts `orth`, `first` … `seventh` or `1` … `8`, case-insensitive,
through [`Layer::from_id`], which reads both spellings off the variant so a reordered layer
cannot leave a stale alias behind. `Layer::id_list` renders the accepted names for the
error message.
A player running it is always the one moved; `[player]` is only consulted from console.
It teleports straight into the layer, so no curse strain is added — unlike a real ascent.

**The `item` and `summon` subcommands were removed.** They were the only callers that made
`mod items` and the kind lookups live, so those now carry `#[expect(dead_code)]` and the
villager tree is dormant. Nothing creates an item or places a custom mob outside
[`CreatureSpawn`](src/events/creature_spawn.rs) until that changes — see § Custom items and
§ Custom villagers for what restores them.

Tab-completion for `<layer>` is **disabled**: attaching it makes the Pumpkin host panic on
the first suggestion request (`block_in_place` from the simulation thread, which has no
Tokio reactor). The `LayerSuggestions` handler is kept in `src/commands/abyss.rs` behind
`cfg(any())` for when that is fixed.

Registration goes through the raw `Context::register_command(command, node)` plus
`Context::register_permission(&Permission)`. Permission nodes come from the trait's
defaulted `Command::permission_node()`, which builds `{PLUGIN_ID}:command.{primary name}`
from [`Command::names`](src/commands/command.rs) — never hand-formatted, so a renamed
command cannot keep its old node. Handler structs implement
`pumpkin_plugin_api::commands::CommandHandler` and return `Ok(1)` on success.

### Curse of the Abyss

`src/events/curse.rs` stores a `curse` strain byte on the player via
`PersistentDataHolder` (namespaced NBT, persisted to disk), so it survives relogs.

- `strain(player)` / `cure(player)` read and clear it
- `apply(player, from)` adds `from.curse()` to the strain, saturating at `MAX_STRAIN`
  (6), then inflicts the symptoms for the **total**
- Ascending several layers in a row therefore compounds: the curse is cumulative
- Reaching `Orth`, or respawning, calls `cure`
- Strain **6 or more** is fatal — `inflict` ends in a catch-all arm rather than one arm per
  value. `apply` caps at `MAX_STRAIN` (6), so in practice the tier is reached and not passed;
  the cap is what keeps strain inside the range the ladder documents, and raising it means
  adding arms to `inflict` in the same change

### Worlds

**`Worlds`** (`src/world/setup.rs`) — creates one Minecraft world per layer on load.

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

`Layer::game_rules()` is the worked example: `Orth` and `First` disable `spawn-monsters`,
the rest return an empty slice. `Worlds::setup` applies the rules on every load, so a world
folder saved with different rules cannot override them.

Minecraft's **difficulty** is a server-wide setting and the plugin API exposes no setter
for it (`Server::get_difficulty` only reads), so peaceful behaviour is expressed as
per-world game rules instead.

### Time of day

Every layer world keeps its own clock, so left alone they drift apart and a descending
player would watch the sun jump. `src/world/clock.rs` copies `Orth`'s clock to the other seven
worlds on a one-minute repeating task, which runs once immediately on load.

`Orth` is the source because it is the city above the Abyss, the layer an operator is most
likely to change the time in, and the start of every descent. Change the time there and the
rest of the Abyss follows within a minute.

The copied value is the full clock, not just the time within a day, so the **day number**
follows as well — `/time set 24000` in `Orth` puts the other layers on day 1 too. A
world's **age** is a separate counter with no setter in the plugin API, so it cannot be
shared; nothing in vanilla depends on age and time of day agreeing.

### Custom mobs

**Minecraft's entity types cannot be extended.** `entity-types.wit` exposes `entity-type` as
a closed enum and there is no registration call anywhere in the plugin WIT, so a "custom
entity" is always **a vanilla entity wearing an identity**. `src/entities/` is the
architecture for that, not a way around it, and it is split by what each trait answers:

- `entity_builder.rs` — the [`EntityBuilder`](src/entities/entity_builder.rs) core every
  kind implements: its base `EntityType`, a stable `id` byte, its visible name, the
  `apply` hook, and [`spawn`](src/entities/entity_builder.rs). `mark`/`id_of` live here too.
- `mobs/mob_builder.rs` — [`MobBuilder`](src/entities/mobs/mob_builder.rs), which extends
  `EntityBuilder` with what makes a creature a **mob**: its `Category`
  (`Passive`/`Docile`/`Aerial`/`Aquatic`/`Hostile`) and the `layers()` it may spawn in.
- `villagers/villager_builder.rs` — [`VillagerBuilder`](src/entities/villagers/villager_builder.rs),
  which extends `EntityBuilder` with what makes a villager a **trader**: its profession and
  the layers its shop stands in. See [Custom villagers](#custom-villagers).

The identity marker is a **byte** in the entity's own persistent data under `PLUGIN_ID`
(`set_byte`/`get_byte`, the same calls `curse.rs` uses), so it survives relogs and restarts
and cannot collide with another plugin's data. It stores `EntityBuilder::id`, not the display
name, so renaming a kind does not orphan entities already spawned. `id_of` returns `None` for
anything this plugin did not spawn, so a handler must not assume every entity is custom. The
number space is **shared by mobs and villagers**, because the marker is.

A kind does everything else in **`EntityBuilder::apply`**: the spawned `LivingEntity` is
handed over and the kind sets attributes with `set_attribute_base` and equips itself with
`set_equipment` per slot. Nothing is a fixed table, so a hand-only mob and a fully armoured
one are just different bodies for that method. `EntityBuilder::spawn` creates the base type,
calls `apply`, and stamps the identity. A villager kind forwards `apply` to
`villager_builder::prepare::<Self>`, which sets the profession and passes a `Villager` handle
to `VillagerBuilder::apply`.

- `Category` is _not_ derived from `EntityType`: the base type says what a mob looks like and
  how the host paths it, the category says what it is for. They usually agree and are allowed
  to disagree, and an unstated category defaults to `Passive` so a new kind is never silently
  treated as something that hunts players.

**Attributes are written to the base value**, matching how every kind sets its stats. The
base is the template and the visible stat is derived from it, so setting a base is
idempotent and never compounds on a repeated call.

**What is not available**, so do not design around it:

- No entity model, texture or hitbox support, and no resource-pack hook. A mob looks like
  its base type; equipment is the only visual lever.
- `get_width`/`get_height` are read-only, so hitbox size cannot be changed.
- Status effects can only be applied to players, so a permanent glowing or speed effect on
  a mob is not expressible.

`MobKind`-style catalogues are the array in `mob_kinds()`, the same way `Layer` holds
per-layer data and `MineInAbyss::events` lists every handler. Adding a mob means adding its
file, its `pub mod` line in `lib.rs`, its `impl EntityBuilder`, its `impl MobBuilder`, and
one entry in `mob_kinds`. It is a function rather than a `const` because a trait method
cannot be called in a constant. `beniguma` is the worked example.

**Kinds are filed by category.** `src/entities/mobs/` holds `mob_builder.rs` plus one folder
per [`Category`](src/entities/mobs/mob_builder.rs) — `aerial`, `aquatic`, `docile`, `hostile`,
`passive` — and a kind lives in the folder matching its own `category()`. The folder is
organisation only: nothing reads it at runtime, so a kind whose category changes must be
**moved** by hand to keep the two agreeing. There is no `mod.rs`; each folder's `pub mod`
list lives inline in the `mod entities` block in `lib.rs`.

**The catalogue as it stands**, with ids and spawn layers:

| Kind              | id  | Base         | Category | Layers                | Hearts | Colour    |
| ----------------- | --- | ------------ | -------- | --------------------- | ------ | --------- |
| `Beniguma`        | 1   | Wolf         | Docile   | First, Second         | 8      | `#824243` |
| `Cyatoria`        | 2   | Cat          | Aerial   | First, Second         | 12.5   | `#3DDFFF` |
| `SmallHammerbeak` | 3   | Parrot       | Aerial   | First, Second         | 9      | `#613F0C` |
| `GiantHammerbeak` | 4   | Parrot       | Aerial   | First, Second, Fourth | 20     | `#613F0C` |
| `Splitjaw`        | 5   | Parrot       | Aerial   | First, Second, Third  | 45     | `#9A1CAD` |
| `Woodcrier`       | 6   | Parrot       | Hostile  | First, Second         | 15     | `#9A1CAD` |
| `ManToyer`        | 7   | Parrot       | Hostile  | First                 | 7.5    | `#D8D8D8` |
| `Silkfang`        | 8   | Parrot       | Hostile  | First, Second         | 11     | `#696112` |
| `Spittleshrub`    | 9   | Parrot       | Hostile  | First, Second, Fourth | 10     | `#696112` |
| `Vinebinder`      | 10  | Creeper      | Hostile  | First, Second         | 10     | `#FFFFFF` |
| `Vineblaster`     | 11  | Creeper      | Hostile  | First, Second         | 8      | `#FFFFFF` |
| `Vinepoofer`      | 12  | Creeper      | Hostile  | First, Second         | 10     | `#FFFFFF` |
| `Fuwagi`          | 13  | Rabbit       | Passive  | First, Second         | 5      | `#E0E0E0` |
| `Okibo`           | 14  | Sniffer      | Passive  | Orth, First           | 20     | `#10DEA7` |
| `Spiralsquid`     | 15  | Squid        | Aquatic  | First, Fourth         | 4      | `#1B91B5` |
| `Chimokami`       | 16  | Fox          | Docile   | First                 | 15     | `#7C3A4A` |
| `CorpseWeeper`    | 17  | Phantom      | Aerial   | First                 | 17.5   | `#D12B26` |
| `Dosetori`        | 18  | Parrot       | Aerial   | First, Second         | 15     | `#59AC7A` |
| `Ottobas`         | 19  | Ravager      | Hostile  | First                 | 45     | `#D8D8D8` |
| `Tesuchi`         | 20  | Turtle       | Passive  | First                 | 10     | `#C0C0C0` |
| `Hisoishi`        | 21  | Endermite    | Docile   | Second, Third         | 12.5   | `#303E77` |
| `Rockwalker`      | 22  | Silverfish   | Docile   | Second, Third         | 12.5   | `#BE6400` |
| `Rohana`          | 23  | Bat          | Aerial   | Second, Fourth        | 5      | `#6C19BF` |
| `Stinger`         | 24  | Bee          | Aerial   | Second, Fourth, Fifth | 8      | `#03AC13` |
| `Inbyo`           | 25  | Spider       | Hostile  | Second                | 12.5   | `#3D3D3D` |
| `Billabonk`       | 26  | Frog         | Passive  | Second                | 10     | `#B754C2` |
| `Neritantan`      | 27  | Rabbit       | Passive  | Second, Third         | 3.5    | `#BDBDBD` |
| `Ikenosuke`       | 28  | TropicalFish | Aquatic  | Second, Third, Fourth | 6      | `#9FF3F3` |
| `Daikonchu`       | 29  | Silverfish   | Docile   | Third                 | 40     | `#C26D23` |
| `Rinsipede`       | 30  | Silverfish   | Docile   | Third                 | 15     | `#FFA500` |
| `Buroguro`        | 31  | Bee          | Aerial   | Third                 | 16     | `#A3F7A2` |
| `Madokajack`      | 32  | Phantom      | Aerial   | Third                 | 20     | `#1C439E` |
| `Squidfly`        | 33  | Bat          | Aerial   | Third                 | 8      | `#85512E` |
| `Himushi`         | 34  | MagmaCube    | Hostile  | Third                 | 30     | `#B22222` |
| `Yomotsubi`       | 35  | Vex          | Hostile  | Third                 | 20     | `#9C0F2C` |
| `Kakatsumuri`     | 36  | Silverfish   | Passive  | Third                 | 2.5    | `#7D93FF` |
| `Sakuranbo`       | 37  | Bee          | Passive  | Third                 | 5      | `#FFB7C5` |
| `Hashofu`         | 38  | Bat          | Aerial   | Fourth, Fifth         | 10     | `#454545` |
| `Onitsuchi`       | 39  | Chicken      | Aerial   | Fourth, Fifth         | 25     | `#5C471D` |
| `Horncrier`       | 40  | Parrot       | Hostile  | Fourth, Fifth         | 30     | `#9A1CAD` |
| `Kuongatari`      | 41  | Spider       | Hostile  | Fourth, Fifth         | 15     | `#1FB53D` |
| `OrbPiercer`      | 42  | CaveSpider   | Hostile  | Fourth                | 90     | `#C74040` |
| `Swarmshocker`    | 43  | Bee          | Hostile  | Fourth, Fifth         | 10     | `#257BEB` |
| `Tachikatana`     | 44  | Spider       | Hostile  | Fourth, Fifth         | 110    | `#00FFFF` |
| `Uibuta`          | 45  | Hoglin       | Hostile  | Fourth, Fifth         | 35     | `#8F2222` |
| `Shroombear`      | 46  | Panda        | Passive  | Fourth                | 12.5   | `#85562D` |
| `Hagamizukin`     | 47  | Cod          | Aquatic  | Fourth, Fifth         | 25     | `#0B6623` |
| `Kazura`          | 48  | GlowSquid    | Aquatic  | Fourth                | 2.5    | `#A9CCCA` |
| `Makihige`        | 49  | Squid        | Aquatic  | Fourth, Fifth         | 30     | `#FF771C` |
| `Spikewalker`     | 50  | Silverfish   | Docile   | Fifth                 | 20     | `#755237` |
| `Titanjaw`        | 51  | Turtle       | Docile   | Fifth                 | 30     | `#3C3B4A` |
| `KagiTachikiri`   | 52  | Vex          | Aerial   | Fifth                 | 25     | `#5468A8` |
| `Chikyuoebi`      | 53  | Spider       | Hostile  | Fifth                 | 42.5   | `#649984` |
| `Octoliar`        | 54  | Squid        | Hostile  | Fifth                 | 40     | `#ADD8E6` |
| `Stingerhead`     | 55  | Phantom      | Hostile  | Fifth                 | 130    | `#9C0F2C` |
| `Swarmhoarder`    | 56  | Bee          | Hostile  | Fifth                 | 35     | `#257BEB` |
| `SporeManta`      | 57  | Phantom      | Aerial   | Fifth                 | 15     | `#6C19BF` |
| `Virinsipede`     | 58  | Silverfish   | Hostile  | Fifth                 | 75     | `#C42D2D` |
| `Shiratsubi`      | 59  | Vex          | Hostile  | Fifth                 | 22.5   | `#9C0F2C` |
| `AbyssalSnail`    | 60  | Silverfish   | Passive  | Fifth                 | 20     | `#785F34` |
| `Ashimite`        | 61  | Goat         | Passive  | Fifth                 | 17.5   | `#2597D9` |
| `Ikonia`          | 62  | Turtle       | Passive  | Fifth                 | 80     | `#3C3B4A` |
| `Demonfish`       | 63  | Pufferfish   | Aquatic  | Fifth                 | 15     | `#1B91B5` |
| `Edusan`          | 64  | Guardian     | Aquatic  | Fifth                 | 40     | `#800080` |
| `Hisomukuro`      | 65  | Squid        | Aquatic  | Fifth                 | 40     | `#C7BCAD` |

Ids are permanent once shipped — they are what tells two kinds on the same base type
apart — so a new kind takes the next free number and never reuses one.

**Names are coloured inline with an exact RGB.** The colours are the hex values the wiki
lists for each mob, so `name()` returns `TextComponent::text(..).color_rgb(rgb(..))`
rather than a `NamedColor`, which has only 16 entries and no cyan, brown or light gray.

**`/abyss summon <mob> [count]` used to be the caller**, and is removed for now, so the
lookup carries `#[expect(dead_code)]`. It resolved a name through [`mob_kinds`](src/lib.rs),
which builds a
[`MobKind`](src/entities/mobs/mob_builder.rs) per kind — id, name, base, layers, category and a spawn
fn pointer — because a kind's associated functions cannot be called through a trait object.
`MobKind::of::<T>(label)` fills the entry from that kind's own methods, so a kind's values
are still written in one place; the label is passed separately because `MobBuilder::name`
returns a `TextComponent` that cannot be read back as a string.

The catalogue is a **function, not a `const`**: `MobKind::of` calls trait methods, which are
not `const`. Adding a kind means adding it to `mob_kinds` in `lib.rs` as well as to the
module list.

**Summoning refuses a layer the kind does not list.** The layer guard is the point of
`MobBuilder::layers` finally having a consumer, and it keeps an operator command from quietly
filling a layer with something that was never meant to live there.

**Names match loosely**: case-insensitive, with spaces and hyphens ignored, so
`small hammerbeak`, `Small Hammerbeak` and `SmallHammerbeak` all resolve.

### Custom villagers

A villager is **not** a mob, and deliberately not in `mob_kinds`. It is not a creature of the
Abyss: it stands behind a counter on an authored map, it is not something a layer spawns, and
`Category` answers a question nobody asks of a shopkeeper. So it has its own builder beside
the mob one rather than a mob kind with an odd category.

- `villager_builder.rs` — [`VillagerBuilder`](src/entities/villagers/villager_builder.rs)
  extends `EntityBuilder` with a `profession()` and `describe(VillagerBuild)`. There is no
  separate layer list: `EntityBuilder` is where an entity states what it is, and a trader has
  no category to declare.
- `villager_trait_builder.rs` — [`VillagerBuild`](src/entities/villagers/villager_trait_builder.rs),
  the chain a trade kind finishes itself with, the same shape as
  [`ItemBuilder`](src/items/item_builder.rs): `of(&villager, profession)` opens it, `.level(..)`
  and `.experience(..)` record what the chain wants, `.apply(|villager| ..)` reaches any setter
  the chain has not grown, and `.build()` — which `villager_builder::prepare` calls — writes
  the profession, level and experience in one go. It **never** writes the identity marker;
  `EntityBuilder::spawn` stamps the entity after `apply` returns. It borrows the `Villager`
  rather than owning it, because `Villager<'_>` is a borrowed wrapper.
- A kind implements **both** traits and writes each answer once. `EntityBuilder` supplies
  `base()` (always `EntityType::Villager`), `id()` (a plain byte, like every mob) and
  `name()`; `VillagerBuilder` supplies the profession and `describe`. The kind's
  `EntityBuilder::apply` forwards to `villager_builder::prepare::<Self>`, which opens the build
  with the kind's profession and finishes it. `EntityBuilder::spawn` is inherited, so a trader
  is placed by the same code as every other custom entity.
- `VillagerKind` is the catalogue entry for one trader — id, name and the spawn fn pointer,
  the same erasure `MobKind` performs. It is smaller than `MobKind` because a trader has no
  category and its profession is applied by the spawn path, so an entry only carries what a
  caller reads. `villager_kinds()` builds them and `villager_kind(name)` resolves one by name.
- **The byte space is shared with the mobs**, so ids must not repeat across the two
  catalogues: mobs hold `1`–`65`, and traders start above them. `Boy` is `66`. There is no
  compile-time guard on this — check both tables when adding a kind.
- **A trader is never a natural spawn.** `CreatureSpawn::choose` filters out any kind whose
  base is `EntityType::Villager`, so a layer's wild spawns can never produce a shopkeeper.
- **Trades cannot be authored yet.** The API reads and writes profession, level and
  experience, but has no call that sets the offers, and `TradeSelect`/`VillagerAcquireTrade`
  carry only an index. A kind therefore leaves its trade table out of `describe` and states
  only the shape of the trader. The wiki's Orth trade table is the source for those offers
  once a setter exists — do not invent a second table elsewhere.
- **Nothing places a trader yet.** `/abyss summon` used to reach one, looked up after the
  mobs and placed wherever the sender stood; with that subcommand gone the villager tree
  carries `#[expect(dead_code)]`. A trader has no layer list to refuse, because where a shop
  stands is a property of the map, not of the kind.

`Boy` is the worked example: `Boy, Lord of Currency`, a librarian placed in `Orth`,
built from the wiki's `Golden Bridge` counter.

### Custom items

Items work exactly like mobs, and for the same reason: `Item` is a **closed enum** of 1658
vanilla items with no registration call in the plugin WIT, so a custom item is a vanilla
item wearing an identity.

- `item_builder.rs` — the [`ItemBuilder`](src/items/item_builder.rs) builder struct a kind
  builds its stack with, and the identity marker. A kind is a **plain constructor**, not a
  trait impl: `Orthcoin::stack()` returns a finished `ItemStack` — the base `Item`, the stable
  `id` byte, its name and its lore. The chain ends at `.build()`, so a caller gets a built,
  stamped stack and never a builder.
- **`ItemBuilder` is a struct, not a trait, deliberately.** A trait earns its keep only when
  something is _generic_ over it — a `[fn() -> ItemStack]` catalogue, a `give<T: Builder>`.
  Nothing is, and items are created by code that already knows the kind, so an inherent
  `stack()` is directly callable with no import of a trait. It comes back when a generic
  consumer does; the villager tree's `VillagerBuilder` is the same shape and exists only
  because a name lookup needs the erasure. The name sits beside
  [`EntityBuilder`](src/entities/entity_builder.rs), and the struct itself exists because an
  `ItemStack` cannot chain.
- **`ItemBuilder` exists because `ItemStack` cannot chain.** The WIT setters return nothing
  and `ItemStack` is not cloneable, so `ItemBuilder::of(id, base).name(..).lore(..).build()`
  is how a kind writes its stack as one expression. It adds no capability — each method is one
  setter call, applied in chain order — and a kind needing a setter it lacks uses
  `.apply(|stack| ..)`, which hands it the live stack. **It never leaves the kind**: `build()`
  is the last step of the kind's constructor, so no caller sees an `ItemBuilder`.
- **The id is a constructor argument, not a chain method**, so a builder cannot exist without
  one: a kind cannot accidentally produce an unmarked stack, and the marker is written by
  `build()` after every other property. A kind writes its number once, in the chain. **The
  count is not an argument**: `of` starts a stack of one, which is what a custom item almost
  always is, and a bundle sets `set_count` through `.apply(|stack| ..)` rather than growing a
  `count()` method — the escape hatch already covers it.
- There is **no `name()` hook**: a name is a property of the stack like any other, so it is
  set in the constructor alongside the lore.
- The marker is `mark`/`id_of`, a **byte** in the stack's own persistent data under
  `PLUGIN_ID`, written with the same `set_byte`/`get_byte` calls `curse.rs` and the mob marker
  use. It survives restarts and cannot collide with another plugin's data.

**Custom data on a stack is an NBT tree, which the API cannot build.** `set_custom_data`
takes an `NbtTree`, and that type is an opaque handle with no constructor, so the
`PersistentDataHolder` `set_byte`/`get_byte` route is the one to use for anything this
plugin stores on an item.

**`Orthcoin` is the worked example**. `/abyss item orthcoin` used to create the stack, put it
in the sender's hand and read the marker back off the stored stack; with that subcommand
removed there is **no caller**, so `mod items` carries `#[expect(dead_code)]` until a real
source of items — loot tables, crafting, a shop — exists. The path itself is unchanged.

**No caller exists now.** With `/abyss item` removed, nothing creates an item, so `mod items`
carries `#[expect(dead_code)]` until a real source does. The path itself is unchanged: a kind
still gets a stamped marker the moment something calls `Orthcoin::stack()`.

**The catalogue as it stands**, with ids and base items:

| Kind       | id  | Base        | Name     | Reachable via |
| ---------- | --- | ----------- | -------- | ------------- |
| `Orthcoin` | 1   | Gold Nugget | Orthcoin | nothing yet   |

Ids are permanent once shipped, for the same reason a mob id is: they are what tells two
kinds on the same base item apart. A new item takes the next free number and never reuses
one — `Orthcoin` is `1`.

There is no `item_kinds()` catalogue function, unlike [`mob_kinds`](src/lib.rs). A catalogue
exists so a **name** can be looked up; items are created by code that already knows which kind
it wants, so a list would be a place to keep in sync for no reader. Adding an item therefore
means its file, its `pub mod` line in the `mod items` block in `lib.rs`, its inherent
`stack()` constructor, and whatever ends up creating it.

**Items have no economy.** `Orthcoin` is a name, a colour, a lore line and a marker — it cannot
be spent, minted from anything, or counted by the plugin. That is deliberate: a currency with
no sink is a promise the design has not made yet, and a stack marker is not a wallet. Real
storage would be per-player persistent data (`set_long`/`get_long`, the same route
`curse.rs` uses for strain), not a property of the item.

#### Each layer is a hand-built map

Every layer world is a **custom map made specifically for this project** — an authored
region with real terrain, structures and points of interest, not a generated world. This
is the central assumption behind the whole design, so keep it in mind whenever you touch
transitions or placement:

- `create_world` only takes a name and dimension — there is no generator, seed or
  level-type argument. The map **is** the world folder on disk, so shipping a layer means
  shipping its world data.
- Worlds are generated **empty**, not left to the default generator: `Worlds::setup`
  registers the [`Void`](src/world/generator.rs) `ChunkGenerator` and attaches it with
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

All eight layers exist as `Layer` variants (identifier, world, name, gradient colour)
with no gameplay content yet. See the table under [Layer System](#layer-system).

There are no per-layer modules or structs: everything a layer needs is on its `Layer`
variant, so adding a layer means adding a variant rather than a file.

`utils::ordinal` also renders the English ordinal-suffix form used for the entry subtitle.

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
├── utils/              # declared as `mod utils` in `lib.rs`: per-category helpers,
│                       # re-exported so callers write `utils::{carry_fall, layer_of}`
│   ├── fall.rs         # `FallCarry` — a descent's distance carried across a boundary
│   ├── general.rs      # `key`, `is_dead`, `exempt_gamemode` — per-player predicates
│   ├── teleport.rs     # `TELEPORTED` — a teleport is not a climb
│   ├── text.rs         # `tell`, `ordinal`, `rgb`
│   ├── transition.rs   # `TRANSITING` — the one-shot latch
│   └── world.rs        # `world_for`, `layer_of`
├── world/              # declared as `mod world` in `lib.rs`: the layer worlds
│   ├── setup.rs        # `Worlds` — one void world per layer, created on load
│   ├── generator.rs    # `Void` chunk generator — layer worlds generate empty
│   └── clock.rs        # Keeps every layer world on `Orth`'s clock
├── entities/           # declared as `mod entities` in `lib.rs`: the plugin's own entities
│   ├── entity_builder.rs # `EntityBuilder` core trait + the identity marker
│   ├── mobs/           # the creatures of the Abyss
│   │   ├── mob_builder.rs # `MobBuilder` (category + layers), `Category`, `MobKind`
│   │   ├── aerial/     # one folder per category; a kind sits in its own
│   │   ├── aquatic/    #   category's folder: aerial 15, aquatic 8, docile 8,
│   │   ├── docile/     #   hostile 23, passive 12 — 65 kinds in all
│   │   ├── hostile/
│   │   └── passive/
│   └── villagers/      # the traders of Orth
│       ├── villager_builder.rs       # `VillagerBuilder`, `VillagerKind`
│       ├── villager_trait_builder.rs # `VillagerBuild` — the chainable trader builder
│       └── boy.rs                    # `Boy` — the worked example, awaiting a caller
├── items/              # declared as `mod items` in `lib.rs`: the plugin's own items
│   ├── item_builder.rs # `ItemBuilder` — the chainable stack builder; no trait
│   └── orthcoin.rs     # `Orthcoin` — the worked example, awaiting a caller
├── commands/
│   ├── abyss.rs        # `/abyss` — ops teleport into a layer, curse-free
│   └── command.rs      # `Command` trait — each command registers itself
├── events/
│   ├── curse.rs        # Curse of the Abyss: strain storage + severity ladder
│   ├── event.rs        # `Event` trait — each handler registers itself
│   ├── bed_spawn.rs    # `BedSpawn` — refuses beds setting a spawn point
│   ├── creature_spawn.rs # `CreatureSpawn` — vanilla spawns become layer kinds
│   ├── fall_damage.rs  # `FallDamage` — carries a descent's fall distance one layer
│   ├── layer_join.rs   # `LayerJoin` — entry title, cure in Orth
│   ├── layer_leave.rs  # `LayerLeave` — void/ceiling teleport + curse
│   ├── layer_respawn.rs# `LayerRespawn` — respawns at Orth's spawn
│   ├── layer_teleport.rs # `LayerTeleport` — teleports are not climbs
```

### Key Conventions

- `unsafe_code` forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`)
- **No `//` line comments.** Explain intent in rustdoc (`///`, `//!`) and in identifier
  and function names instead. Only `///` and `//!` comments belong in `src/`.
- **No `mod.rs`.** Folder modules are declared inline in `lib.rs`, next to `commands`
  and `events`. A folder needs no file of its own: its `//!` docs, `pub mod` list and
  `pub use` re-exports all live in that block — never a `mod.rs`.
- All Clippy warnings enabled; run with `-W clippy::pedantic -D warnings` to catch pedantic lints as errors. (`[lints.clippy] all = "warn"`)
- Permission pattern: `{PLUGIN_ID}:command.{name}` (e.g., `mine_in_abyss:command.example`)
- No config system: no `config.rs`, no `config.toml`, no `serde`/`toml` dependency
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
   `src/events/`, implement `Event::register` beside it, and add one entry to the array in
   `MineInAbyss::events` — prefer one handler for all layers (as `LayerLeave` does) over
   per-layer registration
5. Run `just lint` (and `just validate` before a PR)

## Memory System

This project uses Claude Code's persistent memory in `.claude/memory/`. These files persist across sessions and different PCs. Review `MEMORY.md` for existing context about the user and project.
