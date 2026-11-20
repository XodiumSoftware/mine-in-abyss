//! The Beniguma Fur.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Beniguma Fur a Beniguma leaves behind, built on a phantom membrane.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BenigumaFur;

impl BenigumaFur {
    /// Creates a stamped stack of one Beniguma Fur.
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
        ItemBuilder::of(7, Item::PhantomMembrane)
            .name(TextComponent::text("Beniguma Fur").color_rgb(rgb(0x82, 0x42, 0x43)))
            .lore(vec![TextComponent::text("Beniguma Fur of a Beniguma.")])
            .build()
    }
}
