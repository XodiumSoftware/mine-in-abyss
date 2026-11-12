//! The Stingerhead, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Stingerhead;

impl EntityBuilder for Stingerhead {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        55
    }

    fn name() -> TextComponent {
        TextComponent::text("Stingerhead").color_rgb(rgb(0x9C, 0x0F, 0x2C))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 260.0);
        entity.set_health(260.0);
    }
}

impl MobBuilder for Stingerhead {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
