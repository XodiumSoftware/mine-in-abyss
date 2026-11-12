//! The man-toyer, a hostile flyer of the Top of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A pale flyer that hunts players on the surface.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ManToyer;

impl EntityBuilder for ManToyer {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        7
    }

    fn name() -> TextComponent {
        TextComponent::text("Man-toyer").color_rgb(rgb(0xD8, 0xD8, 0xD8))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 15.0);
        entity.set_health(15.0);
    }
}

impl MobBuilder for ManToyer {
    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
