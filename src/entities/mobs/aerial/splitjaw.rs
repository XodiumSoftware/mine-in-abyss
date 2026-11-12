//! The splitjaw, the heaviest flyer in the upper half of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::{Category, MobBuilder};
use crate::layer::Layer;
use crate::utils::rgb;

/// A large purple flyer, met from the Top of the Abyss down to the Great Fault.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Splitjaw;

impl EntityBuilder for Splitjaw {
    fn base() -> EntityType {
        EntityType::Parrot
    }

    fn id() -> u8 {
        5
    }

    fn name() -> TextComponent {
        TextComponent::text("Splitjaw").color_rgb(rgb(0x9A, 0x1C, 0xAD))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 90.0);
        entity.set_health(90.0);
    }
}

impl MobBuilder for Splitjaw {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second, Layer::Third]
    }

    fn category() -> Category {
        Category::Aerial
    }
}
