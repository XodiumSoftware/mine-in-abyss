//! The Buroguro Lantern.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Buroguro Lantern a Buroguro leaves behind, built on a glowstone dust.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BuroguroLantern;

impl BuroguroLantern {
    /// Creates a stamped stack of one Buroguro Lantern.
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
        ItemBuilder::of(9, Item::GlowstoneDust)
            .name(TextComponent::text("Buroguro Lantern").color_rgb(rgb(0xA3, 0xF7, 0xA2)))
            .lore(vec![TextComponent::text("Buroguro Lantern of a Buroguro.")])
            .build()
    }
}
