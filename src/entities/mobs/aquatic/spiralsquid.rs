//! The spiralsquid, an aquatic mob of the first and fourth layers.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A blue squid, at home wherever a layer's map holds water.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Spiralsquid;

impl Spiralsquid {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Fourth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aquatic;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(15, EntityType::Squid)
            .name(TextComponent::text("Spiralsquid").color_rgb(rgb(0x1B, 0x91, 0xB5)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 8.0);
                    living.set_health(8.0);
                }
            })
            .build(world, position)
    }
}
