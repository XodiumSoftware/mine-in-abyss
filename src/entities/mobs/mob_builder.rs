//! The trait a custom mob implements, and the mob catalogue it is listed in.
//!
//! A mob is a vanilla entity wearing an identity, exactly as
//! [`EntityBuilder`](crate::entities::entity_builder::EntityBuilder) describes: the base
//! type, the name and the marker all come from there. What this trait adds is what makes a
//! mob a *mob* rather than just a custom entity — how it behaves towards players
//! ([`Category`]) and where it may appear ([`layers`](MobBuilder::layers)).
//!
//! Both of those are answers a spawn rule reads, not gameplay code it runs: the layer list is
//! what `/abyss summon` refuses to violate and what
//! [`CreatureSpawn`](crate::events::creature_spawn) consults when it replaces a vanilla
//! spawn, and the category is what a difficulty tweak or a bestiary entry would read. A
//! villager, which is a trader rather than a creature of the Abyss, is built on
//! [`VillagerBuilder`](crate::entities::villagers::villager_builder::VillagerBuilder)
//! instead.

use pumpkin_plugin_api::World;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::Entity;
use strum::{EnumIter, IntoStaticStr};

use crate::entities::entity_builder::EntityBuilder;
use crate::layer::Layer;

/// How a mob kind behaves towards players.
///
/// A separate type from [`EntityType`], which only says *what shape* a mob is: a wolf and a
/// zombie are both spawned as vanilla types, but one is [`Category::Passive`] and the other
/// [`Category::Hostile`]. The category is what a spawn rule, a difficulty tweak or a
/// bestiary entry reads when the vanilla type is not the question.
///
/// Its `id()` is a fixed `snake_case` spelling rather than the variant name, so `"passive"`
/// stays readable in a command argument, a data file or a log line, the same way
/// [`Layer::id`](crate::layer::Layer) does it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Category {
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

impl Category {
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

/// A kind of mob the plugin defines on top of a vanilla one.
///
/// Implemented once per mob kind, on top of [`EntityBuilder`], so a kind's base type, name
/// and marker stay in the shared trait while its category and layers stay here.
pub trait MobBuilder: EntityBuilder {
    /// How this kind behaves towards players.
    ///
    /// Not derived from the base type: the vanilla type says what the mob looks like and how
    /// the host paths it, while the category says what it is for. They usually agree and are
    /// allowed to disagree — a kind built on a wolf can be [`Category::Hostile`] if that is
    /// what it is meant to be.
    ///
    /// The default is [`Category::Passive`], the least aggressive reading: a kind that has
    /// not said otherwise is not silently treated as something that hunts players.
    fn category() -> Category {
        Category::default()
    }

    /// The layers this kind is allowed to spawn in.
    ///
    /// A slice rather than a single [`Layer`] so a kind can live at several depths, and
    /// rather than a predicate so the answer is data a spawn rule can read without running
    /// gameplay code. `&[Layer::First]` for a surface mob, `Layer::ALL` for one that belongs
    /// everywhere.
    ///
    /// The default is empty: a kind that has not said where it lives spawns nowhere, so a new
    /// mob cannot appear somewhere it was never meant to until it states its layers. This is
    /// a constraint on spawning only — [`EntityBuilder::spawn`] itself does not consult it,
    /// because an operator placing a mob deliberately should not be blocked.
    fn layers() -> &'static [Layer] {
        &[]
    }

    /// Returns whether this kind may spawn in `layer`.
    fn spawns_in(layer: Layer) -> bool {
        Self::layers().contains(&layer)
    }
}

/// One entry of the mob catalogue: everything about a kind that can be read without an
/// instance, plus the function that spawns it.
///
/// A kind's own associated functions cannot be called through a trait object or stored as
/// data, so the catalogue spells them out once. [`MobKind::of`] is the bridge: it takes a
/// kind and fills the entry from that kind's own trait methods, so a kind's values are still
/// written in exactly one place.
#[derive(Debug, Clone, Copy)]
pub struct MobKind {
    /// The kind's stable identifier, from [`EntityBuilder::id`].
    pub id: u8,
    /// The name shown above the mob, from [`EntityBuilder::name`].
    pub name: &'static str,
    /// The vanilla type the kind is built on, from [`EntityBuilder::base`].
    pub base: EntityType,
    /// The layers the kind may spawn in, from [`MobBuilder::layers`].
    pub layers: &'static [Layer],
    /// How the kind behaves towards players, from [`MobBuilder::category`].
    pub category: Category,
    /// Spawns the kind, from [`EntityBuilder::spawn`].
    pub spawn: fn(&World, (f64, f64, f64)) -> Entity,
}

impl MobKind {
    /// Builds the catalogue entry for one kind.
    ///
    /// `label` is the kind's display text, which has to be passed separately:
    /// [`EntityBuilder::name`] returns a [`TextComponent`] that cannot be read back as a
    /// string, so the catalogue keeps its own copy rather than trying to reverse the
    /// component.
    pub fn of<T: MobBuilder>(label: &'static str) -> Self {
        Self {
            id: T::id(),
            name: label,
            base: T::base(),
            layers: T::layers(),
            category: T::category(),
            spawn: T::spawn,
        }
    }

    /// Returns whether this kind may spawn in `layer`.
    #[must_use]
    pub fn spawns_in(&self, layer: Layer) -> bool {
        self.layers.contains(&layer)
    }
}
