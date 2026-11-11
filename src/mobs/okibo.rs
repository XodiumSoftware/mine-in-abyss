//! The okibo, a large passive grazer of Orth and the first layer.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A big green sniffer-like creature, found in the city and just below it.
#[derive(Debug, Default, Clone, Copy)]
pub struct Okibo;

impl Mob for Okibo {
    fn base() -> EntityType {
        EntityType::Sniffer
    }

    fn id() -> u8 {
        14
    }

    fn name() -> TextComponent {
        named("Okibo", rgb(0x10, 0xDE, 0xA7))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Orth, Layer::First]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}
