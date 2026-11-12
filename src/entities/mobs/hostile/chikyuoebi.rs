//! The Chikyuoebi, a hostile mob of the Abyss.

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
pub(crate) struct Chikyuoebi;

impl EntityBuilder for Chikyuoebi {
    fn base() -> EntityType {
        EntityType::Spider
    }

    fn id() -> u8 {
        53
    }

    fn name() -> TextComponent {
        TextComponent::text("Chikyuoebi").color_rgb(rgb(0x64, 0x99, 0x84))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 85.0);
        entity.set_health(85.0);
    }
}

impl MobBuilder for Chikyuoebi {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
