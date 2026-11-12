//! The Chimokami, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the first layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Chimokami;

impl MobBuilder for Chimokami {
    fn base() -> EntityType {
        EntityType::Fox
    }

    fn id() -> u8 {
        16
    }

    fn name() -> TextComponent {
        TextComponent::text("Chimokami").color_rgb(rgb(0x7C, 0x3A, 0x4A))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}
