//! The vinebinder, a hostile plant of the first two layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A white creeper-shaped thing that hunts players among the vines.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Vinebinder;

impl EntityBuilder for Vinebinder {
    fn base() -> EntityType {
        EntityType::Creeper
    }

    fn id() -> u8 {
        10
    }

    fn name() -> TextComponent {
        TextComponent::text("Vinebinder").color_rgb(rgb(0xFF, 0xFF, 0xFF))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}

impl MobBuilder for Vinebinder {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
