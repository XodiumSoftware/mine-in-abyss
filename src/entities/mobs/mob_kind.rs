//! One entry of the mob catalogue.
//!
//! The catalogue exists for one consumer — [`CreatureSpawn`](crate::events::creature_spawn),
//! which decides which kind replaces a vanilla spawn in a layer — so an entry carries exactly
//! what that decision reads and nothing more: where the kind may live, how it behaves, and the
//! function that spawns it.
//!
//! A kind states those on itself: `LAYERS` and `CATEGORY` are constants beside its `spawn`
//! constructor, so the values are still written in one place even though the catalogue has to
//! copy them into a plain struct to iterate over.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::world::Entity;

use crate::entities::mobs::mob_category::MobCategory;
use crate::layer::Layer;

/// Everything about a mob kind a spawn rule reads, plus the function that spawns it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct MobKind {
    /// The layers the kind may spawn in, from the kind's `LAYERS`.
    pub(crate) layers: &'static [Layer],
    /// How the kind behaves towards players, from the kind's `CATEGORY`.
    ///
    /// No rule filters on it yet — the creature-spawn rule only asks where a kind may live —
    /// but it is the same answer a difficulty tweak or a bestiary entry would read.
    #[expect(dead_code, reason = "no rule reads a mob's category yet")]
    pub(crate) category: MobCategory,
    /// Spawns the kind, from the kind's own `spawn` constructor.
    pub(crate) spawn: fn(&World, (f64, f64, f64)) -> Entity,
}

impl MobKind {
    /// Builds the catalogue entry for one kind.
    pub(crate) const fn of(
        layers: &'static [Layer],
        category: MobCategory,
        spawn: fn(&World, (f64, f64, f64)) -> Entity,
    ) -> Self {
        Self {
            layers,
            category,
            spawn,
        }
    }

    /// Returns whether this kind may spawn in `layer`.
    #[must_use]
    pub(crate) fn spawns_in(&self, layer: Layer) -> bool {
        self.layers.contains(&layer)
    }
}
