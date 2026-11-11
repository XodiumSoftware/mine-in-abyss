//! The Squidfly, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Squidfly;

impl Mob for Squidfly {
    fn base() -> EntityType {
        EntityType::Bat
    }

    fn id() -> u8 {
        33
    }

    fn name() -> TextComponent {
        named("Squidfly", rgb(0x85, 0x51, 0x2E))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}
