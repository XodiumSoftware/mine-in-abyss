//! The Hisoishi, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the second layer and the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hisoishi;

impl Mob for Hisoishi {
    fn base() -> EntityType {
        EntityType::Endermite
    }

    fn id() -> u8 {
        21
    }

    fn name() -> TextComponent {
        named("Hisoishi", rgb(0x30, 0x3E, 0x77))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Third]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}
