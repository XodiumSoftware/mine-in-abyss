//! The Tachikatana, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Tachikatana;

impl EntityBuilder for Tachikatana {
    fn base() -> EntityType {
        EntityType::Spider
    }

    fn id() -> u8 {
        44
    }

    fn name() -> TextComponent {
        TextComponent::text("Tachikatana").color_rgb(rgb(0x00, 0xFF, 0xFF))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 220.0);
        entity.set_health(220.0);
    }
}

impl MobBuilder for Tachikatana {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
