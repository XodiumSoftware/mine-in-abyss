//! The Orb Piercer Face Fragments, and the
//! [`ItemBuilder`](crate::items::item_builder::ItemBuilder) kind that builds it.
//!
//! The Orb Piercer leaves this behind. To the server it is a gunpowder — the closest vanilla
//! item for what it is — but it carries a name, a colour and lore that make it
//! unmistakable, and its identity marker lets a handler tell it apart from any other
//! gunpowder a player is carrying.
//!
//! It builds itself in one chain of [`ItemBuilder`](crate::items::item_builder::ItemBuilder) calls
//! inside [`stack`](OrbPiercerFaceFragments::stack), starting with a gunpowder and its number — which is
//! how every kind defines itself. Nothing in the plugin creates it yet — the host owns the
//! loot of the entity a kind spawns, and no call in the plugin WIT adds or replaces a drop
//! — so this kind exists to be given, held, recognised and traded.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The Orb Piercer Face Fragments a Orb Piercer leaves behind, built on a gunpowder.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OrbPiercerFaceFragments;

impl OrbPiercerFaceFragments {
    /// Creates a stamped stack of one Orb Piercer Face Fragments.
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
        ItemBuilder::of(73, Item::Gunpowder)
            .name(
                TextComponent::text("Orb Piercer Face Fragments").color_rgb(rgb(0xC7, 0x40, 0x40)),
            )
            .lore(vec![TextComponent::text(
                "Orb Piercer Face Fragments of a Orb Piercer.",
            )])
            .build()
    }
}
