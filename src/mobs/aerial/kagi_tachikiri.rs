//! The Kagi-Tachikiri, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A creature that moves through open air, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct KagiTachikiri;

impl MobBuilder for KagiTachikiri {
    fn base() -> EntityType {
        EntityType::Vex
    }

    fn id() -> u8 {
        52
    }

    fn name() -> TextComponent {
        TextComponent::text("Kagi-Tachikiri").color_rgb(rgb(0x54, 0x68, 0xA8))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 50.0);
        entity.set_health(50.0);
    }
}
