//! The Corpse-Weeper, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the first layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct CorpseWeeper;

impl MobBuilder for CorpseWeeper {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        17
    }

    fn name() -> TextComponent {
        TextComponent::text("Corpse-Weeper").color_rgb(rgb(0xD1, 0x2B, 0x26))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 35.0);
        entity.set_health(35.0);
    }
}
