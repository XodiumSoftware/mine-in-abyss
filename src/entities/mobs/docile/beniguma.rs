//! A docile wolf of the upper layers, showing what a custom mob can express.
//!
//! This is the worked example for a mob kind: it states its [`LAYERS`](Beniguma::LAYERS),
//! [`CATEGORY`](Beniguma::CATEGORY) and `spawn`, and sets its health in one `apply` — which is
//! how every kind defines itself, with no fixed table of multipliers or slots to fill in.
//!
//! In the Abyss this is a docile wolf of the first and second layers: tougher than a plain
//! wolf, so a player can tell it apart from the vanilla mob it is built on and thinks twice
//! before provoking one.

use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::attributes::Attribute;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::{Entity, World};

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;
use crate::utils::rgb;

/// A docile wolf of the Top of the Abyss and the Forest of Temptation.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Beniguma;

impl Beniguma {
    /// The layers this kind may spawn in.
    pub(crate) const LAYERS: &'static [Layer] = &[Layer::First, Layer::Second];

    /// How this kind behaves towards players.
    pub(crate) const CATEGORY: MobCategory = MobCategory::Docile;

    /// Spawns this kind in `world` at `position`.
    pub(crate) fn spawn(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(1, EntityType::Wolf)
            .name(TextComponent::text("Beniguma").color_rgb(rgb(0x82, 0x42, 0x43)))
            .apply(|entity| {
                if let Some(living) = entity.as_living() {
                    living.set_attribute_base(Attribute::MaxHealth, 16.0);
                    living.set_health(16.0);
                }
            })
            .build(world, position)
    }
}
