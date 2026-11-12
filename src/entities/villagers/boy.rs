//! Boy, the Lord of Currency, and Orth's relic buyer.
//!
//! In the Abyss this is the counter of the **Golden Bridge** in Orth's central district,
//! where a delver turns the things they cut off the creatures below into coin. He and his
//! companion Benner are the **Lords of Currency**: they give one Orthcoin for a bundle of a
//! single trophy, and that coin is what every other trader in the city charges.
//!
//! The wiki's trade table is deliberately **not** written here. The plugin API can read and
//! write a villager's profession, level and experience, but it has no call that sets the
//! offers behind them, so a table in this file would be data nothing could deliver. The
//! trades are the part of the [`VillagerBuilder`] chain `build` will grow once the host can be
//! told about them.
//!
//! What it does set is the shape of the trader: a librarian, so its work sound is right, a
//! name over its head so a player can tell it from a village villager, and a first trade
//! level so one is available the moment it is placed.
//!
//! # A trader is not an `EntityBuilder`
//!
//! Unlike a mob, this kind implements no entity trait at all: it is a plain constructor over a
//! [`Villager`], the same way [`Orthcoin`](crate::items::orthcoin::Orthcoin) is a constructor
//! over an `ItemStack`. Nothing spawns it yet — a trader stands where the authored map puts it
//! — so there is no identity byte to stamp and no spawn path to inherit. `place` is all a
//! caller needs: hand it the villager and it describes the trader.

use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::text::TextComponent;

use crate::entities::villagers::villager_builder::VillagerBuilder;
use crate::utils::rgb;

/// Boy, Lord of Currency: the relic buyer of Orth, built on a librarian.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Boy;

impl Boy {
    /// Makes `villager` into Boy: a librarian, named, at trade level one.
    ///
    /// The whole trader is one chain of [`VillagerBuilder`] calls, so nothing about him is
    /// written anywhere else. A caller that wants a different `LivingEntity` converted first
    /// does that itself — this takes a [`Villager`] and nothing else.
    ///
    /// Nothing calls this yet, so the tree carries `#[expect(dead_code)]`; the path is complete
    /// the moment something places a trader.
    pub(crate) fn place(villager: &Villager<'_>) {
        VillagerBuilder::of(villager, VillagerProfession::Librarian)
            .name(TextComponent::text("Boy, Lord of Currency").color_rgb(rgb(0xF0, 0xC4, 0x38)))
            .level(1)
            .experience(0)
            .build();
    }
}
