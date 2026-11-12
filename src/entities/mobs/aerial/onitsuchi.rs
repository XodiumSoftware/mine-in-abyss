//! The Onitsuchi, an aerial mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that moves through open air, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Onitsuchi;

impl EntityBuilder for Onitsuchi {
    fn base() -> EntityType {
        EntityType::Chicken
    }

    fn id() -> u8 {
        39
    }

    fn name() -> TextComponent {
        TextComponent::text("Onitsuchi").color_rgb(rgb(0x5C, 0x47, 0x1D))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 50.0);
        entity.set_health(50.0);
    }
}

impl MobBuilder for Onitsuchi {
    fn layers() -> &'static [Layer] {
        &[Layer::Fourth, Layer::Fifth]
    }

    fn category() -> Category {
        Category::Aerial
    }
}
