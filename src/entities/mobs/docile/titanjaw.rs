//! The Titanjaw, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Titanjaw;

impl EntityBuilder for Titanjaw {
    fn base() -> EntityType {
        EntityType::Turtle
    }

    fn id() -> u8 {
        51
    }

    fn name() -> TextComponent {
        TextComponent::text("Titanjaw").color_rgb(rgb(0x3C, 0x3B, 0x4A))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 60.0);
        entity.set_health(60.0);
    }
}

impl MobBuilder for Titanjaw {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> Category {
        Category::Docile
    }
}
