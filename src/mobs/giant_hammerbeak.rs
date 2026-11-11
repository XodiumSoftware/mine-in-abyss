//! The giant hammerbeak, which reaches from the Top of the Abyss down to the Goblets of
//! Giants.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;

/// The larger hammerbeak, and the one that ranges furthest down the Abyss.
#[derive(Debug, Default, Clone, Copy)]
pub struct GiantHammerbeak;

impl Mob for GiantHammerbeak {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        4
    }

    fn name() -> TextComponent {
        named("Giant Hammerbeak", NamedColor::Gold)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Flying
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}
