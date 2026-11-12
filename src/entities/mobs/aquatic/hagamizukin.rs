//! The Hagamizukin, an aquatic mob of the Abyss.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A creature that lives in water, met in the fourth layer and the fifth layer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Hagamizukin;

impl Hagamizukin {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::Fourth, Layer::Fifth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aquatic;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(47, EntityType::Cod)
            .name(TextComponent::text("Hagamizukin").color_rgb(rgb(0x0B, 0x66, 0x23)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 50.0);
                    living.set_health(50.0);
                }
            })
            .build(world, position)
    }
}
