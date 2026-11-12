//! The Rinsipede, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Rinsipede;

impl EntityBuilder for Rinsipede {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        30
    }

    fn name() -> TextComponent {
        TextComponent::text("Rinsipede").color_rgb(rgb(0xFF, 0xA5, 0x00))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}

impl MobBuilder for Rinsipede {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Docile
    }
}
