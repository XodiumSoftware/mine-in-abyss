//! The Kakatsumuri, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Kakatsumuri;

impl EntityBuilder for Kakatsumuri {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        36
    }

    fn name() -> TextComponent {
        TextComponent::text("Kakatsumuri").color_rgb(rgb(0x7D, 0x93, 0xFF))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 5.0);
        entity.set_health(5.0);
    }
}

impl MobBuilder for Kakatsumuri {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
