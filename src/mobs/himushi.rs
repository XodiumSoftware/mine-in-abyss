//! The Himushi, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Himushi;

impl Mob for Himushi {
    fn base() -> EntityType {
        EntityType::MagmaCube
    }

    fn id() -> u8 {
        34
    }

    fn name() -> TextComponent {
        TextComponent::text("Himushi").color_rgb(rgb(0xB2, 0x22, 0x22))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 60.0);
        entity.set_health(60.0);
    }
}
