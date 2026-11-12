//! The Buroguro, an aerial mob of the Abyss.

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
pub(crate) struct Buroguro;

impl EntityBuilder for Buroguro {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        31
    }

    fn name() -> TextComponent {
        TextComponent::text("Buroguro").color_rgb(rgb(0xA3, 0xF7, 0xA2))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 32.0);
        entity.set_health(32.0);
    }
}

impl MobBuilder for Buroguro {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
