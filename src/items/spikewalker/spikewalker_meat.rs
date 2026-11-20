//! The Spikewalker Meat.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Spikewalker Meat a Spikewalker leaves behind, built on a beef.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SpikewalkerMeat;

impl SpikewalkerMeat {
    /// Creates a stamped stack of one Spikewalker Meat.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone: one drop is one thing, so the stack starts at one and no chain
    /// method contradicts it.
    ///
    /// Nothing calls this yet — the drop table lists it but the host cannot be told to deliver
    /// it — so `mod items` carries `#[expect(dead_code)]` for the whole tree.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(89, Item::Beef)
            .name(TextComponent::text("Spikewalker Meat").color_rgb(rgb(0x75, 0x52, 0x37)))
            .lore(vec![TextComponent::text(
                "Spikewalker Meat of a Spikewalker.",
            )])
            .build()
    }
}
