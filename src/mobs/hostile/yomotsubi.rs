//! The Yomotsubi, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Yomotsubi;

impl MobBuilder for Yomotsubi {
    fn base() -> EntityType {
        EntityType::Vex
    }

    fn id() -> u8 {
        35
    }

    fn name() -> TextComponent {
        TextComponent::text("Yomotsubi").color_rgb(rgb(0x9C, 0x0F, 0x2C))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}
