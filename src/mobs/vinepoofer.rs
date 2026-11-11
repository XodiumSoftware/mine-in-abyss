//! The vinepoofer, a hostile plant of the first two layers.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;

/// A white creeper-shaped thing that hunts players among the vines.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vinepoofer;

impl Mob for Vinepoofer {
    fn base() -> EntityType {
        EntityType::Creeper
    }

    fn id() -> u8 {
        12
    }

    fn name() -> TextComponent {
        named("Vinepoofer", NamedColor::White)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}
