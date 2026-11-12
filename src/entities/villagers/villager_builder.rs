//! The chainable [`VillagerBuilder`] a trader kind describes itself with.
//!
//! The same idea as [`ItemBuilder`](crate::items::item_builder::ItemBuilder): `Villager`'s
//! setters return nothing, so they cannot be chained directly, and a small builder turns the
//! whole description of a trader into one expression. It borrows the [`Villager`] rather than
//! owning it, because `Villager<'_>` is a borrowed wrapper around the spawned mob.
//!
//! It is deliberately small and **self-contained**: it knows only about [`Villager`],
//! [`VillagerProfession`] and the [`TextComponent`] a trader is named with, never about the
//! entity traits that happen to reach it. `villager-data` carries only a profession, a level and
//! an experience count, so the chain is short today, and it exists to keep the shape of a trader
//! in one place — the way [`ItemBuilder`](crate::items::item_builder::ItemBuilder) does for a
//! stack — and to give trades somewhere to go once the host grows a setter for them.
//!
//! Nothing here writes an identity marker or spawns anything: stamping an entity and handing one
//! to this builder are the job of whatever spawns the trader, so this type stays usable from any
//! caller that already holds a [`Villager`].

use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::text::TextComponent;

/// A trader's data, built one property at a time, then written to the villager.
///
/// ```ignore
/// VillagerBuilder::builder(&villager)
///     .name(TextComponent::text("Boy, Lord of Currency"))
///     .level(1)
///     .experience(0)
///     .build();
/// ```
///
/// The profession is a parameter of [`of`](Self::of) rather than a chain method, for the same
/// reason [`ItemBuilder::of`](crate::items::item_builder::ItemBuilder::of) takes the id: a
/// villager without one renders as an unemployed villager, so a kind cannot accidentally
/// produce a trader that has not chosen a job. [`builder`](Self::builder) is the same thing for
/// a caller with nothing to say about the profession; either way a real profession rather than
/// [`VillagerProfession::None`] is the point: `None` has no work sound, no level badge and no
/// trades at all, so it looks like a villager that has not found a job.
///
/// The chain records intent and [`build`](Self::build) is the only thing that touches the
/// villager, mirroring [`ItemBuilder::build`](crate::items::item_builder::ItemBuilder::build):
/// a later `level` overrides an earlier one, exactly as two direct calls would. The exceptions
/// are [`name`](Self::name) and [`apply`](Self::apply), which write immediately because a
/// `TextComponent` cannot be held until `build` and a closure is the caller's own code.
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

    /// Starts a description of `villager` that keeps the profession it already has.
    ///
    /// The counterpart of
    /// [`ItemBuilder::of`](crate::items::item_builder::ItemBuilder::of) for a caller with no
    /// profession in mind — one that is going to set it through [`apply`](Self::apply), or that
    /// only wants to name the trader.
    pub(crate) fn builder(villager: &'a Villager<'a>) -> Self {
        Self::of(villager, villager.get_profession())
    }

    /// Sets the name shown above the trader, and makes it visible.
    ///
    /// The same idea as [`ItemBuilder::name`](crate::items::item_builder::ItemBuilder::name):
    /// a [`TextComponent`], so a kind can colour or style it. Unlike the chain methods this one
    /// writes **immediately**, because a name cannot be held until [`build`](Self::build) —
    /// `TextComponent` is a host resource that is neither `Copy` nor `Clone`, so a builder
    /// carrying one could not stay `Copy`.
    #[must_use]
    pub(crate) fn name(self, name: TextComponent) -> Self {
        let entity = self.villager.as_entity();
        entity.set_custom_name(name);
        entity.set_custom_name_visible(true);
        self
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
    /// This is the only method that has to be called for the chain to have any effect, so a
    /// caller that just wants a vanilla starting trader can open the builder and build it. It
    /// writes nothing else — no identity marker, no spawn — so it is safe to call on a villager
    /// whoever placed it.
    pub(crate) fn build(self) {
        self.villager.set_profession(self.profession);
        self.villager.set_level(self.level);
        self.villager.set_experience(self.experience);
    }
}
