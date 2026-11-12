//! The Madokajack, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Madokajack;

impl EntityBuilder for Madokajack {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        32
    }

    fn name() -> TextComponent {
        TextComponent::text("Madokajack").color_rgb(rgb(0x1C, 0x43, 0x9E))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}

impl MobBuilder for Madokajack {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Aerial
    }
}
