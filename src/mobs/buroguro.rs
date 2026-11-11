//! The Buroguro, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Buroguro;

impl MobBuilder for Buroguro {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        31
    }

    fn name() -> TextComponent {
        TextComponent::text("Buroguro").color_rgb(rgb(0xA3, 0xF7, 0xA2))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 32.0);
        entity.set_health(32.0);
    }
}
