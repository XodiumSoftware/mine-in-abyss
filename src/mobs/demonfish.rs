//! The Demonfish, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Demonfish;

impl MobBuilder for Demonfish {
    fn base() -> EntityType {
        EntityType::Pufferfish
    }

    fn id() -> u8 {
        63
    }

    fn name() -> TextComponent {
        TextComponent::text("Demonfish").color_rgb(rgb(0x1B, 0x91, 0xB5))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aquatic
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}
