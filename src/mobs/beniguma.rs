//! An example mob kind, showing what a custom mob can express.
//!
//! This is the worked example for [`Mob`](super::mob::Mob): it sets its attributes and its
//! equipment in one `apply`, which is how every kind defines itself — there is no fixed
//! table of multipliers or slots to fill in.
//!
//! In the Abyss this is a docile wolf of the upper layers: tougher than a plain wolf, so a
//! player can tell it apart from the vanilla mob it is built on and thinks twice before
//! provoking one. The depth scale in
//! [`Monsters`](crate::events::monsters::Monsters) still applies on top, so the same kind is
//! far more dangerous six layers down.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;

/// A scavenger of the upper layers, built on a wolf.
#[derive(Debug, Default, Clone, Copy)]
pub struct Beniguma;

impl Mob for Beniguma {
    fn base() -> EntityType {
        EntityType::Wolf
    }

    fn id() -> u8 {
        1
    }

    fn name() -> TextComponent {
        TextComponent::text("Beniguma").color_named(NamedColor::Red)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}
