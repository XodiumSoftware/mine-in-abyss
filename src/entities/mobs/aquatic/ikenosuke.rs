//! The Ikenosuke, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the second layer and the third layer and the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Ikenosuke;

impl EntityBuilder for Ikenosuke {
    fn base() -> EntityType {
        EntityType::TropicalFish
    }

    fn id() -> u8 {
        28
    }

    fn name() -> TextComponent {
        TextComponent::text("Ikenosuke").color_rgb(rgb(0x9F, 0xF3, 0xF3))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 12.0);
        entity.set_health(12.0);
    }
}

impl MobBuilder for Ikenosuke {
    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Third, Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aquatic
    }
}
