---
name: add-item
description: Adds a custom item to mine-in-abyss by writing an ItemBuilder impl and wiring it into /abyss item.
---

# Add an Item

A custom item **is a vanilla item wearing an identity**. `Item` is a closed enum with no
registration call anywhere in the plugin WIT, so there is no way to add a new item type.
What makes a stack "custom" is a `u8` marker in its own persistent data, written by
[`ItemBuilder::create`](../../../src/items/item_builder.rs) and read by `id_of`.

For mobs, use the `add-mob` skill instead — the shape is the same, but the trait, the marker
key and the category folders are different.

**Discover, don't assume.** Read these before writing anything:

1. `src/items/item_builder.rs` — the trait, `mark`, `id_of`.
2. `src/items/orthcoin.rs` — the worked example; copy its structure.
3. The `mod items` block in `src/lib.rs`.
4. The `AbyssItem` executor in `src/commands/abyss.rs`.

An earlier version of this project had an `AbyssRelic` on an emerald. It was **removed**;
`Orthcoin` is now the only kind, and the only item `/abyss item` accepts.

## Step 0 — Pick the three values

| Value | Rule |
| ----- | ---- |
| `base()` | A vanilla `Item` variant from `pumpkin_plugin_api::item::Item`. Pick one that looks like the thing. |
| `id()` | The **next free number**, and never a used one again. Check the catalogue table in `AGENTS.md`. |
| `name()` | `TextComponent::text(..).color_rgb(rgb(..))` — exact RGB, not a `NamedColor`. |

**Ids are permanent once shipped.** They are what tells two kinds built on the same base item
apart, so a removed kind's number is not reclaimed. `Orthcoin` kept `2` when `AbyssRelic`
(id `1`) was deleted. The id must also fit in `i8` — `mark` silently writes nothing above
`127`, which would leave an item no handler can recognise.

## Implementation

1. Create `src/items/<name>.rs` with `//!` module docs saying what the item is **in the
   Abyss** and a `///` on the struct:

    ```rust
    use pumpkin_plugin_api::item::Item;
    use pumpkin_plugin_api::text::TextComponent;
    use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

    use super::item_builder::ItemBuilder;
    use crate::utils::rgb;

    /// A coin of Orth, built on a gold nugget.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Orthcoin;

    impl ItemBuilder for Orthcoin {
        fn base() -> Item {
            Item::GoldNugget
        }

        fn id() -> u8 {
            2
        }

        fn name() -> TextComponent {
            TextComponent::text("Orthcoin").color_rgb(rgb(0xF0, 0xC4, 0x38))
        }

        fn apply(stack: &ItemStack) {
            stack.set_lore(vec![
                TextComponent::text("Currency of Orth, the City on the Edge."),
            ]);
        }
    }
    ```

2. **Declare it** in the `mod items` block in `src/lib.rs`. There is no `mod.rs` — the
   submodule list lives inline in that block.
3. **Reach it** from the `AbyssItem` executor in `src/commands/abyss.rs`: accept the spelling,
   call `create(1)`, put the stack in the hand, and read the marker back with `id_of`.

## What `apply` can do

`create` sets the name and stamps the identity for you; `apply` is the hook for everything
else. Verify each call against `wit/item-stack.wit` in the API crate — not memory:

- `set_lore(list<text-component>)` / `add_lore(text-component)`
- enchantments: `add-custom-enchantment`, `get-custom-enchantment-level`
- attribute modifiers: `add-attribute-modifier`

**Do not reach for `set_custom_data` for your own state.** It takes an `NbtTree`, which is an
opaque handle with **no constructor**, so it cannot be built from Rust. Use the
`PersistentDataHolder` `set_byte`/`set_long`/`get_byte` route, which is what `mark`/`id_of`
and `curse.rs` already do.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- `name()` returns an owned `TextComponent` and builds a fresh one per call: it is a host
  resource and cannot be copied or cached.
- An item with **no caller** is dead code. Either wire it into `/abyss item` or into a real
  source (loot table, crafting) in the same change.
- **There is no economy.** A marker is a label, not a wallet; it cannot be spent, minted or
  counted. A real balance is per-player persistent data (`set_long`/`get_long`), which is a
  separate feature — do not imply it exists.

## Verification

Run `just validate` (lint + fmt-check + release build) and `just doc`. The release build is
required: debug builds fail to load in the Pumpkin runtime.

Then update `AGENTS.md` § Custom items: the catalogue table (kind, id, base, name, how to
reach it) and, if the set of `/abyss item` spellings changed, the § Commands row. Add the
file to the § Project Structure tree too.

The marker round trip can only be confirmed in a running Pumpkin server — `/abyss item
<name>` reads it back and reports whether it survived the inventory slot. State in the
summary that in-game confirmation is pending if you have not run the server.

After finishing, summarize the files changed and ask the user whether to commit.
