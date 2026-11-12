//! The Ottobas, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the first layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Ottobas;

impl EntityBuilder for Ottobas {
    fn base() -> EntityType {
        EntityType::Ravager
    }

    fn id() -> u8 {
        19
    }

    fn name() -> TextComponent {
        TextComponent::text("Ottobas").color_rgb(rgb(0xD8, 0xD8, 0xD8))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 90.0);
        entity.set_health(90.0);
    }
}

impl MobBuilder for Ottobas {
    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
