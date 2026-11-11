//! The Spore Manta, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SporeManta;

impl MobBuilder for SporeManta {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        57
    }

    fn name() -> TextComponent {
        TextComponent::text("Spore Manta").color_rgb(rgb(0x6C, 0x19, 0xBF))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}
