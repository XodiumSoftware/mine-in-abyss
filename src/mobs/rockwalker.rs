//! The Rockwalker, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the second layer and the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Rockwalker;

impl Mob for Rockwalker {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        22
    }

    fn name() -> TextComponent {
        named("Rockwalker", rgb(0xBE, 0x64, 0x00))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Third]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}
