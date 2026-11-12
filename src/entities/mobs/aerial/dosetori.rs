//! The Dosetori, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the first layer and the second layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Dosetori;

impl EntityBuilder for Dosetori {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        18
    }

    fn name() -> TextComponent {
        TextComponent::text("Dosetori").color_rgb(rgb(0x59, 0xAC, 0x7A))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}

impl MobBuilder for Dosetori {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Aerial
    }
}
