//! The giant hammerbeak, which reaches from the Top of the Abyss down to the Goblets of
//! Giants.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// The larger hammerbeak, and the one that ranges furthest down the Abyss.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GiantHammerbeak;

impl EntityBuilder for GiantHammerbeak {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        4
    }

    fn name() -> TextComponent {
        TextComponent::text("Giant Hammerbeak").color_rgb(rgb(0x61, 0x3F, 0x0C))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}

impl MobBuilder for GiantHammerbeak {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aerial
    }
}
