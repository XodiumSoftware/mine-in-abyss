//! The Hagamizukin, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A creature that lives in water, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Hagamizukin;

impl MobBuilder for Hagamizukin {
    fn base() -> EntityType {
        EntityType::Cod
    }

    fn id() -> u8 {
        47
    }

    fn name() -> TextComponent {
        TextComponent::text("Hagamizukin").color_rgb(rgb(0x0B, 0x66, 0x23))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aquatic
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 50.0);
        entity.set_health(50.0);
    }
}
