//! The vineblaster, a hostile plant of the first two layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A white creeper-shaped thing that hunts players among the vines.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Vineblaster;

impl EntityBuilder for Vineblaster {
    fn base() -> EntityType {
        EntityType::Creeper
    }

    fn id() -> u8 {
        11
    }

    fn name() -> TextComponent {
        TextComponent::text("Vineblaster").color_rgb(rgb(0xFF, 0xFF, 0xFF))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}

impl MobBuilder for Vineblaster {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> Category {
        Category::Hostile
    }
}
