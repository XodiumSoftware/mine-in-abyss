//! The woodcrier, a hostile flyer of the upper strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A purple flyer that attacks players, unlike the splitjaw it resembles.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Woodcrier;

impl EntityBuilder for Woodcrier {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        6
    }

    fn name() -> TextComponent {
        TextComponent::text("Woodcrier").color_rgb(rgb(0x9A, 0x1C, 0xAD))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 30.0);
        entity.set_health(30.0);
    }
}

impl MobBuilder for Woodcrier {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
