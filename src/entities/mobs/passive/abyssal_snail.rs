//! The Abyssal Snail, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AbyssalSnail;

impl EntityBuilder for AbyssalSnail {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        60
    }

    fn name() -> TextComponent {
        TextComponent::text("Abyssal Snail").color_rgb(rgb(0x78, 0x5F, 0x34))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 40.0);
        entity.set_health(40.0);
    }
}

impl MobBuilder for AbyssalSnail {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
