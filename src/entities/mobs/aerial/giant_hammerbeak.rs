//! The giant hammerbeak, which reaches from the Top of the Abyss down to the Goblets of
//! Giants.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// The larger hammerbeak, and the one that ranges furthest down the Abyss.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GiantHammerbeak;

impl GiantHammerbeak {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second, Layer::Fourth];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Aerial;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(4, EntityType::Parrot)
            .name(TextComponent::text("Giant Hammerbeak").color_rgb(rgb(0x61, 0x3F, 0x0C)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 40.0);
                    living.set_health(40.0);
                }
            })
            .build(world, position)
    }
}
