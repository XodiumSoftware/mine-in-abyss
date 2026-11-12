//! The fuwagi, a harmless white rabbit of the first two layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// A small white rabbit, built on a rabbit so its skittish wandering comes for free.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Fuwagi;

impl MobBuilder for Fuwagi {
    fn base() -> EntityType {
        EntityType::Rabbit
    }

    fn id() -> u8 {
        13
    }

    fn name() -> TextComponent {
        TextComponent::text("Fuwagi").color_rgb(rgb(0xE0, 0xE0, 0xE0))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Passive
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 10.0);
        entity.set_health(10.0);
    }
}
