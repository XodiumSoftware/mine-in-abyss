//! The Hisomukuro, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use super::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hisomukuro;

impl MobBuilder for Hisomukuro {
    fn base() -> EntityType {
        EntityType::Squid
    }

    fn id() -> u8 {
        65
    }

    fn name() -> TextComponent {
        TextComponent::text("Hisomukuro").color_rgb(rgb(0xC7, 0xBC, 0xAD))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aquatic
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 80.0);
        entity.set_health(80.0);
    }
}
