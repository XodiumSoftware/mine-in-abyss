//! The trait a custom villager implements.
//!
//! A villager is a vanilla entity wearing an identity, exactly as
//! [`EntityBuilder`](crate::entities::entity_builder::EntityBuilder) describes: the base type
//! (`EntityType::Villager` for every kind), the name, the identity byte and the spawn path all
//! come from there. What this trait adds is what makes a villager a *trader*: a profession,
//! and — once the host can be told about them — the trades it offers.
//!
//! # Why a villager is not a mob
//!
//! A villager is not a creature of the Abyss: it stands behind a counter on the map, it is
//! not something a layer spawns, and
//! [`Category`](crate::entities::mobs::mob_builder::Category) answers a question nobody asks
//! of a shopkeeper. So it is its own builder rather than a mob kind with an odd category, and
//! [`CreatureSpawn`](crate::events::creature_spawn) refuses to put one where a natural spawn
//! would.
//!
//! # Implementing one
//!
//! A kind implements both traits, and writes each answer once. [`EntityBuilder`] supplies the
//! base type, which is always [`EntityType::Villager`](pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType),
//! the id it stores and the name it shows; [`VillagerBuilder`] supplies the profession and
//! [`apply`](VillagerBuilder::apply). [`EntityBuilder::spawn`] is inherited rather than
//! reimplemented, so a trader is placed by the same code as every other custom entity.
//!
//! # Trades
//!
//! The offers behind a villager cannot be written yet. The plugin API reads and writes a
//! villager's profession, level and experience, but has no call that sets the trades, and the
//! `TradeSelect`/`VillagerAcquireTrade` events carry only an index, never the offer. A kind
//! therefore leaves its trades out of [`apply`](VillagerBuilder::apply) and states only the
//! shape of the trader; the table is added here, not invented elsewhere, once the host grows
//! a setter.

use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::world::{Entity, LivingEntity};

use crate::entities::entity_builder::EntityBuilder;

/// A kind of trader the plugin defines on top of a vanilla villager.
///
/// Implemented once per villager kind, alongside [`EntityBuilder`], so the id, the name and
/// the marker stay in the shared trait while the profession and the trades stay here.
pub(crate) trait VillagerBuilder: EntityBuilder {
    /// The profession this trader wears.
    ///
    /// A real vanilla profession rather than [`VillagerProfession::None`], because the
    /// profession is the only thing about a villager the host renders meaningfully: `None`
    /// has no work sound, no level badge and no trades at all, so a trader wearing it looks
    /// like a villager that has not found a job.
    fn profession() -> VillagerProfession;

    /// Makes `villager` into this kind of trader.
    ///
    /// Runs inside [`EntityBuilder::spawn`] on a handle already confirmed to be a villager, so
    /// a kind does not have to check the type itself. The profession is set before this runs,
    /// from [`profession`](Self::profession); everything else — the level it starts at, its
    /// experience, and one day its trades — is set here.
    fn apply(_villager: &Villager<'_>) {}
}

/// Turns a spawned entity into the trader `T`.
///
/// Called from a kind's [`EntityBuilder::apply`], which is where the shared spawn path hands
/// over the spawned entity. Kept here so every villager kind converts the entity and sets the
/// profession the same way, and so that is never forgotten.
pub(crate) fn prepare<T: VillagerBuilder>(entity: &LivingEntity) {
    let Some(villager) = Villager::from_living(entity) else {
        return;
    };

    villager.set_profession(T::profession());
    <T as VillagerBuilder>::apply(&villager);
}

/// One entry of the villager catalogue: everything about a trader that can be read without an
/// instance, plus the function that spawns it.
///
/// The same erasure [`MobKind`](crate::entities::mobs::mob_builder::MobKind) performs, for the
/// same reason: a kind's associated functions cannot be called through a trait object, so the
/// catalogue spells them out once and [`VillagerKind::of`] fills an entry from that kind's own
/// trait methods.
///
/// It is deliberately smaller than `MobKind`: a trader has no category, and its profession and
/// base type are set by `EntityBuilder::spawn` on the kind itself, so an entry only has to
/// carry what a caller reads — the id to check the marker against, the name to report, and
/// the function to call.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VillagerKind {
    /// The trader's stable identifier, from [`EntityBuilder::id`].
    pub id: u8,
    /// The name shown above the trader, from [`EntityBuilder::name`].
    pub name: &'static str,
    /// Spawns the trader, from [`EntityBuilder::spawn`].
    pub spawn: fn(&World, (f64, f64, f64)) -> Entity,
}

impl VillagerKind {
    /// Builds the catalogue entry for one trader.
    ///
    /// `label` is the trader's display text, passed separately for the same reason
    /// [`MobKind::of`](crate::entities::mobs::mob_builder::MobKind::of) takes one:
    /// [`EntityBuilder::name`] returns a [`TextComponent`](pumpkin_plugin_api::text::TextComponent)
    /// that cannot be read back as a string.
    pub(crate) fn of<T: VillagerBuilder>(label: &'static str) -> Self {
        Self {
            id: T::id(),
            name: label,
            spawn: <T as EntityBuilder>::spawn,
        }
    }
}
