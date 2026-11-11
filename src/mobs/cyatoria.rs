//! A cyan flying cat of the Top of the Abyss and the Forest of Temptation.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;

/// A cyan flying cat, at home in the upper layers.
#[derive(Debug, Default, Clone, Copy)]
pub struct Cyatoria;

impl Mob for Cyatoria {
    fn base() -> EntityType {
        EntityType::Cat
    }

    fn id() -> u8 {
        2
    }

    fn name() -> TextComponent {
        named("Cyatoria", NamedColor::Aqua)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Flying
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}
