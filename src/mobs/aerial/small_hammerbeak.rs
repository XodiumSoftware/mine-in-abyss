//! A small flying hammerbeak, found across the middle strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::layer::Layer;
use crate::mobs::mob_builder::{Category, MobBuilder};
use crate::utils::rgb;

/// The smaller of the two hammerbeaks, the one a player meets first.
#[derive(Debug, Default, Clone, Copy)]
pub struct SmallHammerbeak;

impl MobBuilder for SmallHammerbeak {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        3
    }

    fn name() -> TextComponent {
        TextComponent::text("Small Hammerbeak").color_rgb(rgb(0x61, 0x3F, 0x0C))
    }

    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Aerial
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 18.0);
        entity.set_health(18.0);
    }
}
