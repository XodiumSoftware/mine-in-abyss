//! The Hisoishi, a docile mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_builder::MobBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A peaceful creature that only fights when provoked, met in the second layer and the third layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Hisoishi;

impl EntityBuilder for Hisoishi {
    fn base() -> EntityType {
        EntityType::Endermite
    }

    fn id() -> u8 {
        21
    }

    fn name() -> TextComponent {
        TextComponent::text("Hisoishi").color_rgb(rgb(0x30, 0x3E, 0x77))
    }

    fn apply(entity: &LivingEntity) {
        entity.set_attribute_base(Attribute::MaxHealth, 25.0);
        entity.set_health(25.0);
    }
}

impl MobBuilder for Hisoishi {
    fn layers() -> &'static [Layer] {
        &[Layer::Second, Layer::Third]
    }

    fn category() -> MobCategory {
        MobCategory::Docile
    }
}
