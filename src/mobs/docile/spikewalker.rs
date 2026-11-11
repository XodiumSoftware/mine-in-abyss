//! The Spikewalker, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub struct Spikewalker;

impl MobBuilder for Spikewalker {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        50
    }

    fn name() -> TextComponent {
        TextComponent::text("Spikewalker").color_rgb(rgb(0x75, 0x52, 0x37))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Docile
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}
