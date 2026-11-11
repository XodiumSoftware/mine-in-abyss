//! The woodcrier, a hostile flyer of the upper strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A purple flyer that attacks players, unlike the splitjaw it resembles.
#[derive(Debug, Default, Clone, Copy)]
pub struct Woodcrier;

impl Mob for Woodcrier {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        6
    }

    fn name() -> TextComponent {
        named("Woodcrier", rgb(0x9A, 0x1C, 0xAD))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}
