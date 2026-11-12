//! The Horncrier, a hostile mob of the Abyss.

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
pub(crate) struct Horncrier;

impl EntityBuilder for Horncrier {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        40
    }

    fn name() -> TextComponent {
        TextComponent::text("Horncrier").color_rgb(rgb(0x9A, 0x1C, 0xAD))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 60.0);
        entity.set_health(60.0);
    }
}

impl MobBuilder for Horncrier {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
