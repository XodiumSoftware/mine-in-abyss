//! The shell of an Abyssal Snail, filed beside the.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The shell an Abyssal Snail leaves behind, built on a nautilus shell.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AbyssalSnailShell;

impl AbyssalSnailShell {
    /// Creates a stamped stack of one Abyssal Snail Shell.
    ///
    /// Everything that makes the stack this kind is here: the base item, its number, its name
    /// and its lore. The chain ends at [`build`](ItemBuilder::build), which writes the identity
    /// marker last, so a caller receives a finished stack and never a builder.
    ///
    /// The count is left alone: a shell is one thing, so the stack starts at one and no chain
    /// method contradicts it.
    ///
    /// Nothing calls this yet — the Snail's loot table lists the drop but the host cannot be
    /// told to deliver it — so `mod items` carries `#[expect(dead_code)]` for the whole tree.
    #[must_use]
    pub(crate) fn stack() -> ItemStack {
        ItemBuilder::of(2, Item::NautilusShell)
            .name(TextComponent::text("Abyssal Snail Shell").color_rgb(rgb(0x78, 0x5F, 0x34)))
            .lore(vec![TextComponent::text(
                "The spiral shell of an Abyssal Snail.",
            )])
            .build()
    }
}
