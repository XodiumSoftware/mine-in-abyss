//! The Billabonk, a passive mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A harmless creature, met in the second layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Billabonk;

impl EntityBuilder for Billabonk {
    fn base() -> EntityType {
        EntityType::Frog
    }

    fn id() -> u8 {
        26
    }

    fn name() -> TextComponent {
        TextComponent::text("Billabonk").color_rgb(rgb(0xB7, 0x54, 0xC2))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 20.0);
        entity.set_health(20.0);
    }
}

impl MobBuilder for Billabonk {
    fn layers() -> &'static [Layer] {
        &[Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Passive
    }
}
