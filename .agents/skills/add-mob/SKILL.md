---
name: add-mob
description: Adds a custom mob to mine-in-abyss by writing a MobBuilder impl in the folder for its category and adding it to the mob_kinds catalogue.
---

# Add a Mob

A custom mob **is a vanilla mob wearing an identity**. `EntityType` is a closed enum and the
plugin WIT has no registration call, so there is no way to add a new entity type, model,
texture or hitbox. What makes a mob "custom" is a `u8` marker in its own persistent data,
written by [`EntityBuilder::spawn`](../../../src/entities/entity_builder.rs) and read by `id_of`.

For items, use the `add-item` skill instead — same idea, different trait and marker key.

**Discover, don't assume.** Read these before writing anything:

1. `src/entities/entity_builder.rs` — the shared `EntityBuilder` trait, `mark`/`id_of`.
2. `src/entities/mobs/mob_builder.rs` — `MobBuilder` (adds `layers()`/`category()`), `Category`, `MobKind`.
3. `src/entities/mobs/docile/beniguma.rs` — the worked example; copy its structure.
4. The `mod entities` block **and** `mob_kinds()` in `src/lib.rs`.
5. `AGENTS.md` § Custom mobs for the catalogue and the current highest id.

## Step 0 — Pick the five values

| Value        | Rule                                                                                          |
| ------------ | --------------------------------------------------------------------------------------------- |
| `base()`     | A vanilla `EntityType` variant. Says what it looks like and how the host paths it.            |
| `id()`       | The **next free number**, never a used one again. Check the catalogue in `AGENTS.md`.         |
| `name()`     | `TextComponent::text(..).color_rgb(rgb(..))` — exact RGB, not a `NamedColor`.                 |
| `category()` | One of `Passive` (default), `Docile`, `Aerial`, `Aquatic`, `Hostile`.                         |
| `layers()`   | Which layers it may spawn in. **Empty by default — a kind that says nothing spawns nowhere.** |

**Ids are permanent once shipped.** They are what tells two kinds on the same base type apart,
so a removed kind's number is not reclaimed. The id must fit in `i8` — `mark` silently writes
nothing above `127`.

## Step 1 — File it in its category's folder

`src/entities/mobs/` holds `mob_builder.rs` plus one folder per `Category`: `aerial`, `aquatic`,
`docile`, `hostile`, `passive`. A kind lives in the folder matching its **own `category()`**.

The folder is organisation only — nothing reads it at runtime — so a kind whose category
changes must be **moved by hand** to keep the two agreeing.

## Step 2 — Write the kind

`src/entities/mobs/<category>/<snake_case_name>.rs`:

```rust
//! A docile wolf of the upper layers, showing what a custom mob can express.
//!
//! In the Abyss this is a docile wolf of the first and second layers: tougher than a plain
//! wolf, so a player can tell it apart from the vanilla mob it is built on.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A docile wolf of the Top of the Abyss and the Forest of Temptation.
#[derive(Debug, Default, Clone, Copy)]
pub struct Beniguma;

impl EntityBuilder for Beniguma {
    fn base() -> EntityType {
        EntityType::Wolf
    }

    fn id() -> u8 {
        1
    }

    fn name() -> TextComponent {
        TextComponent::text("Beniguma").color_rgb(rgb(0x82, 0x42, 0x43))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}

impl MobBuilder for Beniguma {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Docile
    }
}
```

## Step 3 — Register it in two places

1. **`pub mod <file>;`** in the matching category block of the `mod entities { mod mobs { .. } }` block in `lib.rs`.
2. **One entry in `mob_kinds()`** in `lib.rs`, in id order:

    ```rust
    entities::mobs::mob_builder::MobKind::of::<entities::mobs::docile::beniguma::Beniguma>("Beniguma"),
    ```

The catalogue is a **function, not a `const`**, because `MobKind::of` calls trait methods,
which are not `const`. `MobKind::of` fills the entry from the kind's own methods, so the values
are still written in exactly one place — but the display label is passed separately because
`name()` returns a `TextComponent` that cannot be read back as a string.

## What `apply` can do

Everything the kind does to the spawned entity goes here — there is no fixed table.

- `set_attribute_base(Attribute::.., value)` — the **base** value, not the current one. The
  base is the template and the visible stat derives from it, so setting a base is idempotent
  and never compounds.
- `set_equipment(EquipmentSlot::MainHand, ..)` — per slot, so equip only what the kind means
  to carry. Armour slots left untouched stay vanilla.
- Raising `MaxHealth` **also** needs `set_health(..)` to match, or the mob arrives damaged.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- `category()` is **not** derived from `base()`: the vanilla type says what a mob looks like,
  the category says what it is for. They may legitimately disagree.
- **Monster stats do not scale with depth.** A mob is exactly what its type says at every
  layer — do not add a depth multiplier.
- What the API cannot do: no model/texture/hitbox, no resource pack, no size change, and
  status effects apply to players only. Do not design around them.
- `layers()` constrains **spawning**, not `spawn()` — an operator may place a mob anywhere
  deliberately. It still decides what `CreatureSpawn` may replace a vanilla spawn with, so a
  missing layer means the kind never appears naturally.
- An entity cannot change layers. Nothing carries a mob down when a player descends.

## Verification

Run `just validate` (lint + fmt-check + release build) and `just doc`. The release build is
required: debug builds fail to load in the Pumpkin runtime.

Then update `AGENTS.md` § Custom mobs: the catalogue table (kind, id, base, category, layers,
hearts, colour) and the per-category counts on the `src/entities/mobs/` tree line. Add the file to the
§ Project Structure tree.

In game, a kind appears in a layer whose game rules allow natural spawning, through
`CreatureSpawn`. There is no summon command at present, so that spawn rule is the only way to
see one. State in the summary that in-game confirmation is pending if you have not run the
server.

After finishing, summarize the files changed and ask the user whether to commit.
