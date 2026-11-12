//! The chainable [`VillagerBuilder`] a trader kind describes itself with.
//!
//! The same idea as [`ItemBuilder`](crate::items::item_builder::ItemBuilder): `Villager`'s
//! setters return nothing, so they cannot be chained directly, and a small builder turns the
//! whole description of a trader into one expression. Unlike the item builder this one never
//! writes an identity marker — [`EntityBuilder::spawn`](crate::entities::entity_builder::EntityBuilder::spawn)
//! stamps the entity after `apply` returns — and it borrows the [`Villager`] rather than owning
//! it, because `Villager<'_>` is a borrowed wrapper around the spawned mob.
//!
//! It is deliberately small: `villager-data` carries only a profession, a level and an
//! experience count, so the chain is short today. It exists to keep the shape of a kind in one
//! place, the way [`ItemBuilder`](crate::items::item_builder::ItemBuilder) does for a stack,
//! and to give trades somewhere to go once the host grows a setter for them.
//!
//! There is deliberately no villager trait. A kind is a plain [`EntityBuilder`] whose
//! `apply` opens this builder itself, exactly as `Orthcoin::stack` is a constructor rather than
//! an [`ItemBuilder`](crate::items::item_builder::ItemBuilder) impl: nothing is generic over
//! villager kinds — there is no catalogue to erase them into — so a trait would add an import
//! and an indirection without giving a caller anything.
//!
//! # Why a villager is not a mob
//!
//! A villager is not a creature of the Abyss: it stands behind a counter on the map, it is not
//! something a layer spawns, and
//! [`MobCategory`](crate::entities::mobs::mob_category::MobCategory) answers a question nobody asks of
//! a shopkeeper. So it is described by this builder rather than by a mob kind with an odd
//! category, and [`CreatureSpawn`](crate::events::creature_spawn) refuses to put one where a
//! natural spawn would.

use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;

/// A trader's data, built one property at a time, then written to the villager.
///
/// ```ignore
/// VillagerBuilder::of(&villager, VillagerProfession::Librarian)
///     .level(1)
///     .experience(0)
///     .build();
/// ```
///
/// The profession is a parameter of [`of`](Self::of) rather than a chain method, for the same
/// reason [`ItemBuilder::of`](crate::items::item_builder::ItemBuilder::of) takes the id: a
/// villager without one renders as an unemployed villager, so a kind cannot accidentally
/// produce a trader that has not chosen a job. A real profession rather than
/// [`VillagerProfession::None`] is the point: `None` has no work sound, no level badge and no
/// trades at all, so it looks like a villager that has not found a job.
///
/// The chain records intent and [`build`](Self::build) is the only thing that touches the
/// villager, mirroring [`ItemBuilder::build`](crate::items::item_builder::ItemBuilder::build):
/// a later `level` overrides an earlier one, exactly as two direct calls would. A kind needing
/// a setter this type has not grown yet reaches the live villager through
/// [`apply`](Self::apply).
#[derive(Clone, Copy)]
pub(crate) struct VillagerBuilder<'a> {
    villager: &'a Villager<'a>,
    profession: VillagerProfession,
    level: u8,
    experience: u32,
}

impl<'a> VillagerBuilder<'a> {
    /// Starts a trader description for `villager`, claiming `profession`.
    ///
    /// Nothing is written here: [`build`](Self::build) applies every property at once, so the
    /// villager is only touched once the kind has said everything it wants to say.
    ///
    /// The level starts at `1` and the experience at `0`, matching what
    /// [`Villager::get_level`] reports for a villager that has never traded, so a kind that
    /// omits either gets the vanilla starting trader rather than a level `0` one.
    pub(crate) fn of(villager: &'a Villager<'a>, profession: VillagerProfession) -> Self {
        Self {
            villager,
            profession,
            level: 1,
            experience: 0,
        }
    }

    /// Sets the trading level, `1`–`5`.
    #[must_use]
    pub(crate) fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    /// Sets the trading experience the trader starts with.
    #[must_use]
    pub(crate) fn experience(mut self, experience: u32) -> Self {
        self.experience = experience;
        self
    }

    /// Runs `extend` over the live villager, for anything the chain has not grown a method for.
    ///
    /// The escape hatch, matching
    /// [`ItemBuilder::apply`](crate::items::item_builder::ItemBuilder::apply): a kind that needs
    /// a setter this type has not grown calls it here instead of reaching past the builder.
    /// Unlike the chain methods it writes **immediately** and in chain order, so a later
    /// [`build`](Self::build) can still override what it set.
    #[must_use]
    pub(crate) fn apply(self, extend: impl FnOnce(&Villager<'_>)) -> Self {
        extend(self.villager);
        self
    }

    /// Writes the profession, level and experience, finishing the trader.
    ///
    /// The identity marker is **not** written here — the shared spawn path stamps the entity
    /// once `apply` has returned — so this only has to set what a villager is.
    pub(crate) fn build(self) {
        self.villager.set_profession(self.profession);
        self.villager.set_level(self.level);
        self.villager.set_experience(self.experience);
    }
}
