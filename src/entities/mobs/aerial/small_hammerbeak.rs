//! A small flying hammerbeak, found across the middle strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// The smaller of the two hammerbeaks, the one a player meets first.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SmallHammerbeak;

impl EntityBuilder for SmallHammerbeak {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        3
    }

    fn name() -> TextComponent {
        TextComponent::text("Small Hammerbeak").color_rgb(rgb(0x61, 0x3F, 0x0C))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 18.0);
        entity.set_health(18.0);
    }
}

impl MobBuilder for SmallHammerbeak {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
