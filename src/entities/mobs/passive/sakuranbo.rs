//! The Sakuranbo, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Sakuranbo;

impl EntityBuilder for Sakuranbo {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        37
    }

    fn name() -> TextComponent {
        TextComponent::text("Sakuranbo").color_rgb(rgb(0xFF, 0xB7, 0xC5))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 10.0);
        entity.set_health(10.0);
    }
}

impl MobBuilder for Sakuranbo {
    fn layers() -> &'static [Layer] {
        &[Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
