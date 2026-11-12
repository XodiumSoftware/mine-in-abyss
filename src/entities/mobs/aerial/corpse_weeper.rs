//! The Corpse-Weeper, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the first layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct CorpseWeeper;

impl EntityBuilder for CorpseWeeper {
    fn base() -> EntityType {
        EntityType::Phantom
    }

    fn id() -> u8 {
        17
    }

    fn name() -> TextComponent {
        TextComponent::text("Corpse-Weeper").color_rgb(rgb(0xD1, 0x2B, 0x26))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 35.0);
        entity.set_health(35.0);
    }
}

impl MobBuilder for CorpseWeeper {
    fn layers() -> &'static [Layer] {
        &[Layer::First]
    }

    fn category() -> MobCategory {
        MobCategory::Aerial
    }
}
