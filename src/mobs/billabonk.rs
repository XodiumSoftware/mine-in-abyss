//! The Billabonk, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the second layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Billabonk;

impl Mob for Billabonk {
    fn base() -> EntityType {
        EntityType::Frog
    }

    fn id() -> u8 {
        26
    }

    fn name() -> TextComponent {
        named("Billabonk", rgb(0xB7, 0x54, 0xC2))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}
