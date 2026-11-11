//! The Shroombear, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A harmless creature, met in the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Shroombear;

impl MobBuilder for Shroombear {
    fn base() -> EntityType {
        EntityType::Panda
    }

    fn id() -> u8 {
        46
    }

    fn name() -> TextComponent {
        TextComponent::text("Shroombear").color_rgb(rgb(0x85, 0x56, 0x2D))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fourth]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}
