//! The giant hammerbeak, which reaches from the Top of the Abyss down to the Goblets of
//! Giants.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

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
        named("Giant Hammerbeak", rgb(0x61, 0x3F, 0x0C))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}
