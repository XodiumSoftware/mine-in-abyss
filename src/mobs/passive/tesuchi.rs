//! The Tesuchi, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A harmless creature, met in the first layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Tesuchi;

impl MobBuilder for Tesuchi {
    fn base() -> EntityType {
        EntityType::Turtle
    }

    fn id() -> u8 {
        20
    }

    fn name() -> TextComponent {
        TextComponent::text("Tesuchi").color_rgb(rgb(0xC0, 0xC0, 0xC0))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}
