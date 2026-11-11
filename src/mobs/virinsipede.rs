//! The Virinsipede, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Virinsipede;

impl Mob for Virinsipede {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        58
    }

    fn name() -> TextComponent {
        TextComponent::text("Virinsipede").color_rgb(rgb(0xC4, 0x2D, 0x2D))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 150.0);
        entity.set_health(150.0);
    }
}
