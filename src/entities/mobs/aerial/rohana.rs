//! The Rohana, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the second layer and the fourth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Rohana;

impl EntityBuilder for Rohana {
    fn base() -> EntityType {
        EntityType::Bat
    }

    fn id() -> u8 {
        23
    }

    fn name() -> TextComponent {
        TextComponent::text("Rohana").color_rgb(rgb(0x6C, 0x19, 0xBF))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 10.0);
        entity.set_health(10.0);
    }
}

impl MobBuilder for Rohana {
    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Fourth]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
