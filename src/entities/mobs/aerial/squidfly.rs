//! The Squidfly, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Squidfly;

impl EntityBuilder for Squidfly {
    fn base() -> EntityType {
        EntityType::Bat
    }

    fn id() -> u8 {
        33
    }

    fn name() -> TextComponent {
        TextComponent::text("Squidfly").color_rgb(rgb(0x85, 0x51, 0x2E))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}

impl MobBuilder for Squidfly {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
