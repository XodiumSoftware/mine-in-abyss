//! The Virinsipede, a hostile mob of the Abyss.

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
pub(crate) struct Virinsipede;

impl EntityBuilder for Virinsipede {
    fn base() -> EntityType {
        EntityType::Silverfish
    }

    fn id() -> u8 {
        58
    }

    fn name() -> TextComponent {
        TextComponent::text("Virinsipede").color_rgb(rgb(0xC4, 0x2D, 0x2D))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 150.0);
        entity.set_health(150.0);
    }
}

impl MobBuilder for Virinsipede {
    fn layers() -> &'static [Layer] {
        &[Layer::Fifth]
    }

    fn category() -> MobCategory {
        MobCategory::Hostile
    }
}
