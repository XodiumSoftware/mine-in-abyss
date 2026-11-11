//! The Edusan, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Edusan;

impl MobBuilder for Edusan {
    fn base() -> EntityType {
        EntityType::Guardian
    }

    fn id() -> u8 {
        64
    }

    fn name() -> TextComponent {
        TextComponent::text("Edusan").color_rgb(rgb(0x80, 0x00, 0x80))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aquatic
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 80.0);
        entity.set_health(80.0);
    }
}
