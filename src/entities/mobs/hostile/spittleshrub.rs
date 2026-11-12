//! The spittleshrub, a hostile flyer of the upper strata.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A yellow flyer that hunts players in the first and second layers.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Spittleshrub;

impl Spittleshrub {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second, Layer::Fourth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Hostile;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(9, EntityType::Parrot)
            .name(TextComponent::text("Spittleshrub").color_rgb(rgb(0x69, 0x61, 0x12)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 20.0);
                    living.set_health(20.0);
                }
            })
            .build(world, position)
    }
}
