//! The gunk an Abyssal Snail leaves behind, and a third.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The gunk an Abyssal Snail leaves behind, built on a slimeball.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AbyssalSnailGunk;

impl AbyssalSnailGunk {
    /// Creates a stamped stack of one Abyssal Snail Gunk.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone even though the Snail drops one or two: the drop table's `1-2`
    /// is how many stacks the host should hand out, not how many one stack holds. A caller that
    /// wants a pair builds one stack per item, as it would for any other kind.
    ///
    /// Nothing calls this yet — the Snail's loot table lists the drop but the host cannot be
    /// told to deliver it — so `mod items` carries `#[expect(dead_code)]` for the whole tree.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(3, Item::SlimeBall)
            .name(TextComponent::text("Abyssal Snail Gunk").color_rgb(rgb(0x78, 0x5F, 0x34)))
            .lore(vec![TextComponent::text(
                "Viscous gunk left by an Abyssal Snail.",
            )])
            .build()
    }
}
