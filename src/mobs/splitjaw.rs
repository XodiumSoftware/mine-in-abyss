//! The splitjaw, the heaviest flyer in the upper half of the Abyss.

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;

/// A large purple flyer, met from the Top of the Abyss down to the Great Fault.
#[derive(Debug, Default, Clone, Copy)]
pub struct Splitjaw;

impl Mob for Splitjaw {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        5
    }

    fn name() -> TextComponent {
        named("Splitjaw", NamedColor::LightPurple)
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Third]
    }

    fn category() -> Category {
        Category::Flying
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 90.0);
        entity.set_health(90.0);
    }
}
