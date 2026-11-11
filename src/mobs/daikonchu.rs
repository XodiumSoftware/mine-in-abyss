//! The Daikonchu, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Daikonchu;

impl Mob for Daikonchu {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        29
    }

    fn name() -> TextComponent {
        TextComponent::text("Daikonchu").color_rgb(rgb(0xC2, 0x6D, 0x23))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 80.0);
        entity.set_health(80.0);
    }
}
