//! The Orb Piercer, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OrbPiercer;

impl EntityBuilder for OrbPiercer {
    fn base() -> EntityType {
        EntityType::CaveSpider
    }

    fn id() -> u8 {
        42
    }

    fn name() -> TextComponent {
        TextComponent::text("Orb Piercer").color_rgb(rgb(0xC7, 0x40, 0x40))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 180.0);
        entity.set_health(180.0);
    }
}

impl MobBuilder for OrbPiercer {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
