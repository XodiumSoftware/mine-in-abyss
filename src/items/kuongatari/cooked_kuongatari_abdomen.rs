//! The cooked abdomen of a Kuongatari, and the
//! [`ItemBuilder`](crate::items::item_builder::ItemBuilder) kind that builds it.
//!
//! The Kuongatari leaves this behind once it has been cooked. To the server it is cooked beef —
//! the same meat the raw abdomen is built on, cooked — but it carries a name, a colour and lore
//! that make it unmistakable, and its identity marker lets a handler tell it apart from any
//! other cooked beef a player is carrying.
//!
//! It builds itself in one chain of [`ItemBuilder`](crate::items::item_builder::ItemBuilder)
//! calls inside [`stack`](CookedKuongatariAbdomen::stack), starting with the cooked beef and its
//! number — which is how every kind defines itself. Nothing in the plugin creates it yet — the
//! host owns the loot of the entity a kind spawns, and no call in the plugin WIT adds or
//! replaces a drop — so this kind exists to be given, held, recognised and traded.
//!
//! The wiki's Orth trades list it beside the raw abdomen at Boy's counter. It is a separate kind
//! from [`KuongatariAbdomen`](super::kuongatari_abdomen::KuongatariAbdomen) because it is a
//! different stack a player can hold, not a state of the raw one.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use crate::items::item_builder::ItemBuilder;
use crate::utils::rgb;

/// The cooked abdomen a Kuongatari leaves behind, built on cooked beef.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct CookedKuongatariAbdomen;

impl CookedKuongatariAbdomen {
    /// Creates a stamped stack of one Cooked Kuongatari Abdomen.
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
        ItemBuilder::of(124, Item::CookedBeef)
            .name(TextComponent::text("Cooked Kuongatari Abdomen").color_rgb(rgb(0x1F, 0xB5, 0x3D)))
            .lore(vec![TextComponent::text(
                "A cooked abdomen taken from a Kuongatari.",
            )])
            .build()
    }
}
