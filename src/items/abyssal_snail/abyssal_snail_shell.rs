//! The shell of an Abyssal Snail, filed beside the
//! [`AbyssalSnailGunk`](super::abyssal_snail_gunk::AbyssalSnailGunk) the same creature leaves
//! behind.
//!
//! Both are plain
//! [`ItemBuilder`](crate::items::item_builder::ItemBuilder) kinds, built the same way as
//! [`Orthcoin`](crate::items::orthcoin::Orthcoin) — this one is the shell half of the pair.
//!
//! The Abyssal Snail is the passive shell of the fifth layer, and its shell drops about seven
//! times in ten. To the server this is a nautilus shell — the closest vanilla shell — but it
//! carries a name, a colour and lore that make it unmistakable, and its identity marker lets a
//! handler tell it apart from any other nautilus shell a player is carrying.
//!
//! It builds itself in one chain of [`ItemBuilder`](crate::items::item_builder::ItemBuilder) calls
//! inside [`stack`](AbyssalSnailShell::stack), starting with the nautilus shell and its number
//! — which is how every kind defines itself. Nothing in the plugin creates it yet — the host
//! owns the loot of the entity a kind spawns, and no call in the plugin WIT adds or replaces a
//! drop — so this kind exists to be given, held and recognised.

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
