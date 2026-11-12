//! A cyan flying cat of the Top of the Abyss and the Forest of Temptation.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A cyan flying cat, at home in the upper layers.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cyatoria;

impl MobBuilder for Cyatoria {
    fn base() -> EntityType {
        EntityType::Cat
    }

    fn id() -> u8 {
        2
    }

    fn name() -> TextComponent {
        TextComponent::text("Cyatoria").color_rgb(rgb(0x3D, 0xDF, 0xFF))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}
