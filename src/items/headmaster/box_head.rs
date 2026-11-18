//! The Box Head, sold at the counter of Headmaster.
//!
//! The [`ItemBuilder`](crate::items::item_builder::ItemBuilder) kind behind one row of that
//! trader's offers: to the server it is a carved pumpkin — the closest vanilla item for a wearable
//! head — but it carries a name, a colour and lore that make it unmistakable, and its identity
//! marker lets a handler tell it apart from any other carved pumpkin a player is carrying.
//!
//! It builds itself in one chain of [`ItemBuilder`](crate::items::item_builder::ItemBuilder)
//! calls inside [`stack`](BoxHead::stack), starting with a carved pumpkin and its number — which
//! is how every kind defines itself. Nothing in the plugin creates it yet — the plugin WIT has
//! no call that sets a villager's offers, so no trade can deliver it — so this kind exists to be
//! given, held and recognised.
//!
//! 12 Orthcoin buys it.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Box Head sold by Headmaster, built on a carved pumpkin.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BoxHead;

impl BoxHead {
    /// Creates a stamped stack of one Box Head.
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
        ItemBuilder::of(121, Item::CarvedPumpkin)
            .name(TextComponent::text("Box Head").color_rgb(rgb(0xC0, 0x39, 0x2B)))
            .lore(vec![TextComponent::text(
                "A carved box shell worn over the head.",
            )])
            .build()
    }
}
