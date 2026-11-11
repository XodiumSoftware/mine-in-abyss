//! The Stingerhead, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stingerhead;

impl Mob for Stingerhead {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        55
    }

    fn name() -> TextComponent {
        named("Stingerhead", rgb(0x9C, 0x0F, 0x2C))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 260.0);
        entity.set_health(260.0);
    }
}
