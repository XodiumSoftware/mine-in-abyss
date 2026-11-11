//! The Neritantan, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the second layer and the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Neritantan;

impl Mob for Neritantan {
    fn base() -> EntityType {
        EntityType::Rabbit
    }

    fn id() -> u8 {
        27
    }

    fn name() -> TextComponent {
        TextComponent::text("Neritantan").color_rgb(rgb(0xBD, 0xBD, 0xBD))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Third]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 7.0);
        entity.set_health(7.0);
    }
}
