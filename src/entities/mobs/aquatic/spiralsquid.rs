//! The spiralsquid, an aquatic mob of the first and fourth layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A blue squid, at home wherever a layer's map holds water.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Spiralsquid;

impl EntityBuilder for Spiralsquid {
    fn base() -> EntityType {
        EntityType::Squid
    }

    fn id() -> u8 {
        15
    }

    fn name() -> TextComponent {
        TextComponent::text("Spiralsquid").color_rgb(rgb(0x1B, 0x91, 0xB5))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 8.0);
        entity.set_health(8.0);
    }
}

impl MobBuilder for Spiralsquid {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Fourth]
    }

    fn category() -> MobCategory {
        MobCategory::Aquatic
    }
}
