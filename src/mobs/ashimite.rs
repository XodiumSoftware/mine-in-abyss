//! The Ashimite, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ashimite;

impl Mob for Ashimite {
    fn base() -> EntityType {
        EntityType::Goat
    }

    fn id() -> u8 {
        61
    }

    fn name() -> TextComponent {
        named("Ashimite", rgb(0x25, 0x97, 0xD9))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 35.0);
        entity.set_health(35.0);
    }
}
