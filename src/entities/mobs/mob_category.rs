//! How a mob kind behaves towards players.
//!
//! Kept apart from [`MobBuilder`](crate::entities::mobs::mob_builder::MobBuilder) because it is
//! a question about the kind's behaviour rather than about how a kind is built, and because it
//! is the folder layout of `src/entities/mobs` — a kind lives in the folder matching its own
//! `category()`.

use strum::{EnumIter, IntoStaticStr};

/// How a mob kind behaves towards players.
///
/// A separate type from [`EntityType`](pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType),
/// which only says *what shape* a mob is: a wolf and a zombie are both spawned as vanilla
/// types, but one is [`MobCategory::Passive`] and the other [`MobCategory::Hostile`]. The
/// category is what a spawn rule, a difficulty tweak or a bestiary entry reads when the vanilla
/// type is not the question.
///
/// Its `id()` is a fixed `snake_case` spelling rather than the variant name, so `"passive"`
/// stays readable in a command argument, a data file or a log line, the same way
/// [`Layer::id`](crate::layer::Layer) does it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MobCategory {
    /// Never attacks a player unprovoked; will fight back or not, per kind.
    #[default]
    Passive,
    /// May be bred or tamed, and only harms players when provoked.
    Docile,
    /// Moves through the air, so its pathing ignores the ground.
    Aerial,
    /// Swims, so it belongs in water rather than on the layer's map.
    Aquatic,
    /// Attacks players on sight.
    Hostile,
}

impl MobCategory {
    /// Every category, in the order used for listings.
    pub const ALL: [Self; 5] = [
        Self::Passive,
        Self::Docile,
        Self::Aerial,
        Self::Aquatic,
        Self::Hostile,
    ];

    /// This category's stable lowercase identifier, e.g. `"hostile"`.
    ///
    /// Read off the variant by `strum`, never hand-written, so renaming a variant cannot
    /// leave a stale spelling behind.
    #[must_use]
    pub fn id(self) -> &'static str {
        self.into()
    }

    /// Returns whether a mob of this category hunts players.
    #[must_use]
    pub fn is_hostile(self) -> bool {
        matches!(self, Self::Hostile)
    }
}
