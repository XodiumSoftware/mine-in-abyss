//! The Stinger, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the second layer and the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stinger;

impl MobBuilder for Stinger {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        24
    }

    fn name() -> TextComponent {
        TextComponent::text("Stinger").color_rgb(rgb(0x03, 0xAC, 0x13))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}
