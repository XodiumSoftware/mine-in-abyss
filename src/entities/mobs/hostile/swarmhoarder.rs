//! The Swarmhoarder, a hostile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that attacks players on sight, met in the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Swarmhoarder;

impl EntityBuilder for Swarmhoarder {
    fn base() -> EntityType {
        EntityType::Bee
    }

    fn id() -> u8 {
        56
    }

    fn name() -> TextComponent {
        TextComponent::text("Swarmhoarder").color_rgb(rgb(0x25, 0x7B, 0xEB))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 70.0);
        entity.set_health(70.0);
    }
}

impl MobBuilder for Swarmhoarder {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
