//! A small flying hammerbeak, found across the middle strata.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;

/// The smaller of the two hammerbeaks, the one a player meets first.
#[derive(Debug, Default, Clone, Copy)]
pub struct SmallHammerbeak;

impl Mob for SmallHammerbeak {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        3
    }

    fn name() -> TextComponent {
        named("Small Hammerbeak", NamedColor::Gold)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Flying
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 18.0);
        entity.set_health(18.0);
    }
}
