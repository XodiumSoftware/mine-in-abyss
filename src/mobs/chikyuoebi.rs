//! The Chikyuoebi, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Chikyuoebi;

impl Mob for Chikyuoebi {
    fn base() -> EntityType {
        EntityType::Spider
    }

    fn id() -> u8 {
        53
    }

    fn name() -> TextComponent {
        named("Chikyuoebi", rgb(0x64, 0x99, 0x84))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 85.0);
        entity.set_health(85.0);
    }
}
