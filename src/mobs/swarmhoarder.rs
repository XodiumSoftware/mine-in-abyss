//! The Swarmhoarder, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob::{Category, Mob};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Swarmhoarder;

impl Mob for Swarmhoarder {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        56
    }

    fn name() -> TextComponent {
        TextComponent::text("Swarmhoarder").color_rgb(rgb(0x25, 0x7B, 0xEB))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Hostile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 70.0);
        entity.set_health(70.0);
    }
}
