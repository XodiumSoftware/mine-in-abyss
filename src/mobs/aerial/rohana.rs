//! The Rohana, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A creature that moves through open air, met in the second layer and the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Rohana;

impl MobBuilder for Rohana {
    fn base() -> EntityType {
        EntityType::Bat
    }

    fn id() -> u8 {
        23
    }

    fn name() -> TextComponent {
        TextComponent::text("Rohana").color_rgb(rgb(0x6C, 0x19, 0xBF))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 10.0);
        entity.set_health(10.0);
    }
}
