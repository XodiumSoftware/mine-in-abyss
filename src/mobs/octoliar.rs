//! The Octoliar, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Octoliar;

impl Mob for Octoliar {
    fn base() -> EntityType {
        EntityType::Squid
    }

    fn id() -> u8 {
        54
    }

    fn name() -> TextComponent {
        TextComponent::text("Octoliar").color_rgb(rgb(0xAD, 0xD8, 0xE6))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 80.0);
        entity.set_health(80.0);
    }
}
