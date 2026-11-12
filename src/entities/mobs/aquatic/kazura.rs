//! The Kazura, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Kazura;

impl EntityBuilder for Kazura {
    fn base() -> EntityType {
        EntityType::GlowSquid
    }

    fn id() -> u8 {
        48
    }

    fn name() -> TextComponent {
        TextComponent::text("Kazura").color_rgb(rgb(0xA9, 0xCC, 0xCA))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 5.0);
        entity.set_health(5.0);
    }
}

impl MobBuilder for Kazura {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth]
    }

    fn category() -> Category {
        Category::Aquatic
    }
}
