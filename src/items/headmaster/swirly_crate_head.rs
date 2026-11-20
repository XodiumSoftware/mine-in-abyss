//! The Swirly Crate Head, sold at the counter of Headmaster.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Swirly Crate Head sold by Headmaster, built on a carved pumpkin.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SwirlyCrateHead;

impl SwirlyCrateHead {
    /// Creates a stamped stack of one Swirly Crate Head.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone: one head is one thing, so the stack starts at one and no chain
    /// method contradicts it.
    ///
    /// Nothing calls this yet — no trade can be delivered, so `mod items` carries
    /// `#[expect(dead_code)]` for the whole tree.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(120, Item::CarvedPumpkin)
            .name(TextComponent::text("Swirly Crate Head").color_rgb(rgb(0xC0, 0x39, 0x2B)))
            .lore(vec![TextComponent::text(
                "A carved crate shell worn over the head.",
            )])
            .build()
    }
}
