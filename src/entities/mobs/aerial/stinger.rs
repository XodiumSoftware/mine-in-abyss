//! The Stinger, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the second layer and the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Stinger;

impl EntityBuilder for Stinger {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        24
    }

    fn name() -> TextComponent {
        TextComponent::text("Stinger").color_rgb(rgb(0x03, 0xAC, 0x13))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}

impl MobBuilder for Stinger {
    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Fourth, Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
