//! The Kazura, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob, named};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Kazura;

impl Mob for Kazura {
    fn base() -> EntityType {
        EntityType::GlowSquid
    }

    fn id() -> u8 {
        48
    }

    fn name() -> TextComponent {
        named("Kazura", rgb(0xA9, 0xCC, 0xCA))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aquatic
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 5.0);
        entity.set_health(5.0);
    }
}
