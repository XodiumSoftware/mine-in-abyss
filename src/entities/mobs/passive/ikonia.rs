//! The Ikonia, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Ikonia;

impl EntityBuilder for Ikonia {
    fn base() -> EntityType {
        EntityType::Turtle
    }

    fn id() -> u8 {
        62
    }

    fn name() -> TextComponent {
        TextComponent::text("Ikonia").color_rgb(rgb(0x3C, 0x3B, 0x4A))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 160.0);
        entity.set_health(160.0);
    }
}

impl MobBuilder for Ikonia {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
