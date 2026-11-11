//! Orth's currency, and the worked example for [`ItemBuilder`](super::item_builder::ItemBuilder).
//!
//! In the Abyss this is the coin of Orth, the city on the edge: a plain gold nugget to the
//! server, but it carries a name, a colour and lore that make it unmistakable, and its
//! marker lets a handler tell it apart from any other gold nugget a player is carrying.
//!
//! It sets its lore in one `apply`, which is how every kind defines itself — there is no fixed
//! table of components to fill in. Nothing in the plugin spends it yet — there is no economy,
//! and no shop to spend it in — so this kind exists to be given, held and recognised.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use super::item_builder::ItemBuilder;
use crate::utils::rgb;

/// A coin of Orth, built on a gold nugget.
#[derive(Debug, Default, Clone, Copy)]
pub struct Orthcoin;

impl ItemBuilder for Orthcoin {
    fn base() -> Item {
        Item::GoldNugget
    }

    fn id() -> u8 {
        2
    }

    fn name() -> TextComponent {
        TextComponent::text("Orthcoin").color_rgb(rgb(0xF0, 0xC4, 0x38))
    }

    fn apply(stack: &ItemStack) {
        stack.set_lore(vec![
            TextComponent::text("Currency of Orth, the City on the Edge."),
            TextComponent::text("It buys passage, and not much else."),
        ]);
    }
}
