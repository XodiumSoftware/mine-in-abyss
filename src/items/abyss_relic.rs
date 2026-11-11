//! An example item kind, showing what a custom item can express.
//!
//! This is the worked example for [`ItemBuilder`](super::item_builder::ItemBuilder): it sets
//! its lore in one `apply`, which is how every kind defines itself — there is no fixed table
//! of components to fill in.
//!
//! In the Abyss this is a relic pulled from the Forest of Temptation: a plain emerald to the
//! server, but it carries a name, a colour and lore that make it unmistakable, and its marker
//! lets a handler tell it apart from any other emerald a player is carrying.

use pumpkin_plugin_api::item::Item;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::item_stack::ItemStack;

use super::item_builder::ItemBuilder;
use crate::utils::rgb;

/// A relic of the Forest of Temptation, built on an emerald.
#[derive(Debug, Default, Clone, Copy)]
pub struct AbyssRelic;

impl ItemBuilder for AbyssRelic {
    fn base() -> Item {
        Item::Emerald
    }

    fn id() -> u8 {
        1
    }

    fn name() -> TextComponent {
        TextComponent::text("Abyss Relic").color_rgb(rgb(0x5B, 0xA8, 0x5B))
    }

    fn apply(stack: &ItemStack) {
        stack.set_lore(vec![
            TextComponent::text("A relic of the Forest of Temptation."),
            TextComponent::text("It hums, faintly."),
        ]);
    }
}
