---
name: add-item
description: Adds a custom item to mine-in-abyss by writing an inherent constructor that builds an `ItemBuilder` stack, and giving it a caller.
---

# Add an Item

A custom item **is a vanilla item wearing an identity**. `Item` is a closed enum with no
registration call anywhere in the plugin WIT, so there is no way to add a new item type.
What makes a stack "custom" is a `u8` marker in its own persistent data, written by
`ItemBuilder::build` in [`item_builder.rs`](../../../src/items/item_builder.rs) and read by
`id_of`.

For mobs, use the `add-mob` skill instead — the shape is the same, but the builder trait, the
marker key and the category folders are different.

**Discover, don't assume.** Read these before writing anything:

1. `src/items/item_builder.rs` — `ItemBuilder`, `mark`, `id_of`. It is a **struct, not a
   trait**.
2. `src/items/orthcoin.rs` — the worked example; copy its structure.
3. The `mod items` block in `src/lib.rs`.
4. `AGENTS.md` § Custom items for the catalogue and the id table.

An earlier version of this project had an `AbyssRelic` on an emerald. It was **removed**;
`Orthcoin` is the only kind, and its id `1`.

## Step 0 — Pick the values

| Value     | Rule                                                                                                                                                                                                                                                                        |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| the id    | The **next free number**, passed inline to `ItemBuilder::of`, and never a used one again. Check the catalogue table in `AGENTS.md`.                                                                                                                                         |
| the stack | Built in the kind's `stack()` with `ItemBuilder::of(id, base)` — the count starts at one. The name and lore go on the chain: `TextComponent::text(..).color_rgb(rgb(..))` — exact RGB, not a `NamedColor`. A kind that really is a bundle sets `set_count` through `.apply( | stack | ..)`. |

There is **no trait and no `id()`, `name()` or `base()` hook**. A kind is a plain constructor
that describes itself in one chain, so the base item, its number, name and lore are all written
in one place; a kind that skips the name produces an unnamed stack that looks vanilla, while an
unmarked stack is impossible because the id is a constructor argument. A trait would only earn
its place once something is generic over item kinds (a catalogue, a `give<T>`); nothing is, so
the constructor is directly callable with no import.

**Ids are permanent once shipped.** They are what tells two kinds built on the same base item
apart, so a removed kind's number is not reclaimed. The id must also fit in `i8` — `mark`
silently writes nothing above `127`, which would leave an item no handler can recognise.

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

    impl Orthcoin {
        #[must_use]
        pub fn stack() -> ItemStack {
            ItemBuilder::of(2, Item::GoldNugget)
                .name(TextComponent::text("Orthcoin").color_rgb(rgb(0xF0, 0xC4, 0x38)))
                .lore(vec![
                    TextComponent::text("Currency of Orth, the City on the Edge."),
                ])
                .build()
        }
    }
    ```

2. **Declare it** in the `mod items` block in `src/lib.rs`. There is no `mod.rs` — the
   submodule list lives inline in that block.
3. **Give it a caller.** There is no `/abyss item` subcommand at present, so an item with no
   other source is dead code and `mod items` already carries `#[expect(dead_code)]`. Wire the
   kind into a real source — a loot table, a craft, a shop — in the same change where you can.

## What `stack()` can do

`stack()` returns a finished stack, the name and base item included. `ItemBuilder` covers
the common properties and `build()` ends the chain. Verify each call against
`wit/item-stack.wit` in the API crate — not memory:

- through `ItemBuilder`: `.name(..)`, `.lore(..)`, then `.build()`
- through `.apply(|stack| ..)`: `add_custom_enchantment`, `add_attribute_modifier`, `set_count`,
  and any other setter the builder has not grown yet — it hands you the live `&mut ItemStack`,
  which is the escape hatch that replaces reaching past the builder

**Do not reach for `set_custom_data` for your own state.** It takes an `NbtTree`, which is an
opaque handle with **no constructor**, so it cannot be built from Rust. Use the
`PersistentDataHolder` `set_byte`/`set_long`/`get_byte` route, which is what `mark`/`id_of`
and `curse.rs` already do.

## Constraints

- **No `//` line comments.** Explain intent in `///` and `//!`, or in names.
- Never log with `println!` — use `tracing::{info, warn, error}`.
- `TextComponent` is a host resource that cannot be copied, so build a fresh one per call
  (`TextComponent::text(..)` is cheap) rather than caching it.
- An item with **no caller** is dead code. Either wire it into a real source (loot table,
  crafting, a shop) or accept that the whole `mod items` tree is dormant; see step 3.
- **There is no economy.** A marker is a label, not a wallet; it cannot be spent, minted or
  counted. A real balance is per-player persistent data (`set_long`/`get_long`), which is a
  separate feature — do not imply it exists.

## Verification

Run `just validate` (lint + fmt-check + release build) and `just doc`. The release build is
required: debug builds fail to load in the Pumpkin runtime.

Then update `AGENTS.md` § Custom items: the catalogue table (kind, id, base, name, how to
reach it). Add the file to the § Project Structure tree too.

The marker can only be confirmed in a running Pumpkin server, by reading the stack back with
`id_of` after it has been through an inventory slot. With no command that creates an item,
that check has to be written wherever the kind is first given out. State in the summary that
in-game confirmation is pending if you have not run the server.

After finishing, summarize the files changed and ask the user whether to commit.
