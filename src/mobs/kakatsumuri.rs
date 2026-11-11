//! The Kakatsumuri, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Kakatsumuri;

impl Mob for Kakatsumuri {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        36
    }

    fn name() -> TextComponent {
        named("Kakatsumuri", rgb(0x7D, 0x93, 0xFF))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 5.0);
        entity.set_health(5.0);
    }
}
