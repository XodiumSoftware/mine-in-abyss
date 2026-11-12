//! A cyan flying cat of the Top of the Abyss and the Forest of Temptation.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A cyan flying cat, at home in the upper layers.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cyatoria;

impl Cyatoria {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aerial;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(2, EntityType::Cat)
            .name(TextComponent::text("Cyatoria").color_rgb(rgb(0x3D, 0xDF, 0xFF)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 25.0);
                    living.set_health(25.0);
                }
            })
            .build(world, position)
    }
}
