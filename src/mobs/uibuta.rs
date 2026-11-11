//! The Uibuta, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Uibuta;

impl MobBuilder for Uibuta {
    fn base() -> EntityType {
        EntityType::Hoglin
    }

    fn id() -> u8 {
        45
    }

    fn name() -> TextComponent {
        TextComponent::text("Uibuta").color_rgb(rgb(0x8F, 0x22, 0x22))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 70.0);
        entity.set_health(70.0);
    }
}
