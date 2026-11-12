//! The Shroombear, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Shroombear;

impl EntityBuilder for Shroombear {
    fn base() -> EntityType {
        EntityType::Panda
    }

    fn id() -> u8 {
        46
    }

    fn name() -> TextComponent {
        TextComponent::text("Shroombear").color_rgb(rgb(0x85, 0x56, 0x2D))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}

impl MobBuilder for Shroombear {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
