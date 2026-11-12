//! The Inbyo, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the second layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Inbyo;

impl EntityBuilder for Inbyo {
    fn base() -> EntityType {
        EntityType::Spider
    }

    fn id() -> u8 {
        25
    }

    fn name() -> TextComponent {
        TextComponent::text("Inbyo").color_rgb(rgb(0x3D, 0x3D, 0x3D))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}

impl MobBuilder for Inbyo {
    fn layers() -> &'static [Layer] {
        &[Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
