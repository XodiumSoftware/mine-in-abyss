//! The okibo, a large passive grazer of Orth and the first layer.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A big green sniffer-like creature, found in the city and just below it.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Okibo;

impl EntityBuilder for Okibo {
    fn base() -> EntityType {
        EntityType::Sniffer
    }

    fn id() -> u8 {
        14
    }

    fn name() -> TextComponent {
        TextComponent::text("Okibo").color_rgb(rgb(0x10, 0xDE, 0xA7))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}

impl MobBuilder for Okibo {
    fn layers() -> &'static [Layer] {
        &[Layer::Orth, Layer::First]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
