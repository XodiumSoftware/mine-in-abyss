//! The Virinsipede Segments.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Virinsipede Segments a Virinsipede leaves behind, built on a string.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct VirinsipedeSegments;

impl VirinsipedeSegments {
    /// Creates a stamped stack of one Virinsipede Segments.
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
        ItemBuilder::of(114, Item::String)
            .name(TextComponent::text("Virinsipede Segments").color_rgb(rgb(0xC4, 0x2D, 0x2D)))
            .lore(vec![TextComponent::text(
                "Virinsipede Segments of a Virinsipede.",
            )])
            .build()
    }
}
