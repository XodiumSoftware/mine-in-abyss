//! Orth's currency.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use super::item_builder::ItemBuilder;
use crate::utils::rgb;

/// A coin of Orth, built on a gold nugget.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Orthcoin;

impl Orthcoin {
    /// Creates a stamped stack of one Orthcoin.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone: a coin is one thing, so the stack starts at one and no chain
    /// method contradicts it. A kind that only makes sense as a single item gets that for free.
    ///
    /// Nothing calls this yet — there is no real source of items, so `mod items` carries
    /// `#[expect(dead_code)]` — but the path is complete the moment something does.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(1, Item::GoldNugget)
            .name(TextComponent::text("Orthcoin").color_rgb(rgb(0xF0, 0xC4, 0x38)))
            .lore(vec![TextComponent::text(
                "Currency of Orth, the City on the Edge.",
            )])
            .build()
    }
}
