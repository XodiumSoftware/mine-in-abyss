//! The spittleshrub, a hostile flyer of the upper strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A yellow flyer that hunts players in the first and second layers.
#[derive(Debug, Default, Clone, Copy)]
pub struct Spittleshrub;

impl MobBuilder for Spittleshrub {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        9
    }

    fn name() -> TextComponent {
        TextComponent::text("Spittleshrub").color_rgb(rgb(0x69, 0x61, 0x12))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}
