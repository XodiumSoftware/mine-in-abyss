//! The Hashofu, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Hashofu;

impl EntityBuilder for Hashofu {
    fn base() -> EntityType {
        EntityType::Bat
    }

    fn id() -> u8 {
        38
    }

    fn name() -> TextComponent {
        TextComponent::text("Hashofu").color_rgb(rgb(0x45, 0x45, 0x45))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}

impl MobBuilder for Hashofu {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
