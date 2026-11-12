//! The Kagi-Tachikiri, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct KagiTachikiri;

impl EntityBuilder for KagiTachikiri {
    fn base() -> EntityType {
        EntityType::Vex
    }

    fn id() -> u8 {
        52
    }

    fn name() -> TextComponent {
        TextComponent::text("Kagi-Tachikiri").color_rgb(rgb(0x54, 0x68, 0xA8))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 50.0);
        entity.set_health(50.0);
    }
}

impl MobBuilder for KagiTachikiri {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
