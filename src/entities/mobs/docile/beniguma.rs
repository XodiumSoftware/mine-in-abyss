//! A docile wolf of the upper layers, showing what a custom mob can express.
//!
//! This is the worked example for [`MobBuilder`](super::entity_builder::MobBuilder): it sets its health in one `apply`,
//! which is how every kind defines itself — there is no fixed table of multipliers or slots
//! to fill in.
//!
//! In the Abyss this is a docile wolf of the first and second layers: tougher than a plain
//! wolf, so a player can tell it apart from the vanilla mob it is built on and thinks twice
//! before provoking one.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A docile wolf of the Top of the Abyss and the Forest of Temptation.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Beniguma;

impl EntityBuilder for Beniguma {
    fn base() -> EntityType {
        EntityType::Wolf
    }

    fn id() -> u8 {
        1
    }

    fn name() -> TextComponent {
        TextComponent::text("Beniguma").color_rgb(rgb(0x82, 0x42, 0x43))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 16.0);
        entity.set_health(16.0);
    }
}

impl MobBuilder for Beniguma {
    fn layers() -> &'static [Layer] {
        &[Layer::First, Layer::Second]
    }

    fn category() -> MobCategory {
        MobCategory::Docile
    }
}
