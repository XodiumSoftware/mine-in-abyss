//! The man-toyer, a hostile flyer of the Top of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A pale flyer that hunts players on the surface.
#[derive(Debug, Default, Clone, Copy)]
pub struct ManToyer;

impl Mob for ManToyer {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        7
    }

    fn name() -> TextComponent {
        named("Man-toyer", rgb(0xD8, 0xD8, 0xD8))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 15.0);
        entity.set_health(15.0);
    }
}
