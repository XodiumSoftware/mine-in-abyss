//! The silkfang, a hostile flyer of the upper strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A yellow flyer that hunts players in the first and second layers.
#[derive(Debug, Default, Clone, Copy)]
pub struct Silkfang;

impl Mob for Silkfang {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        8
    }

    fn name() -> TextComponent {
        named("Silkfang", rgb(0x69, 0x61, 0x12))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 22.0);
        entity.set_health(22.0);
    }
}
