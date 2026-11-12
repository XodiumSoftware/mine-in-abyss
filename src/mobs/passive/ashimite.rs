//! The Ashimite, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A harmless creature, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Ashimite;

impl MobBuilder for Ashimite {
    fn base() -> EntityType {
        EntityType::Goat
    }

    fn id() -> u8 {
        61
    }

    fn name() -> TextComponent {
        TextComponent::text("Ashimite").color_rgb(rgb(0x25, 0x97, 0xD9))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 35.0);
        entity.set_health(35.0);
    }
}
