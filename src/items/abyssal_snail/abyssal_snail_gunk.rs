//! The gunk an Abyssal Snail leaves behind, and a third
//! [`ItemBuilder`](crate::items::item_builder::ItemBuilder) kind beside
//! [`Orthcoin`](crate::items::orthcoin::Orthcoin) and
//! [`AbyssalSnailShell`](super::abyssal_snail_shell::AbyssalSnailShell).
//!
//! The Abyssal Snail is the passive shell of the fifth layer, and its gunk drops in ones and
//! twos about seven times in ten. To the server this is a slimeball — the closest vanilla
//! substance — but it carries a name, a colour and lore that make it unmistakable, and its
//! identity marker lets a handler tell it apart from any other slimeball a player is carrying.
//!
//! It builds itself in one chain of [`ItemBuilder`](crate::items::item_builder::ItemBuilder) calls
//! inside [`stack`](AbyssalSnailGunk::stack), starting with the slimeball and its number —
//! which is how every kind defines itself. Nothing in the plugin creates it yet — the host
//! owns the loot of the entity a kind spawns, and no call in the plugin WIT adds or replaces a
//! drop — so this kind exists to be given, held, recognised and traded.

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
