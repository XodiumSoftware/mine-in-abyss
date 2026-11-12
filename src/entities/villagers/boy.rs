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
//! trades are the part of a trader [`apply`](VillagerBuilder::apply) will grow once the host
//! can be told about them.
//!
//! What it does set is the shape of the trader: a librarian, so its work sound is right, a
//! name over its head so a player can tell it from a village villager, and a first trade
//! level so one is available the moment it is placed.

use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::wit::pumpkin::plugin::entity_types::EntityType;
use pumpkin_plugin_api::world::LivingEntity;

use crate::entities::entity_builder::EntityBuilder;
use crate::entities::villagers::villager_builder::{VillagerBuilder, prepare};
use crate::utils::rgb;

/// Boy, Lord of Currency: the relic buyer of Orth, built on a librarian.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Boy;

impl EntityBuilder for Boy {
    fn base() -> EntityType {
        EntityType::Villager
    }

    fn id() -> u8 {
        66
    }

    fn name() -> TextComponent {
        TextComponent::text("Boy, Lord of Currency").color_rgb(rgb(0xF0, 0xC4, 0x38))
    }

    fn apply(entity: &LivingEntity) {
        prepare::<Self>(entity);
    }
}

impl VillagerBuilder for Boy {
    fn profession() -> VillagerProfession {
        VillagerProfession::Librarian
    }

    fn apply(villager: &Villager<'_>) {
        villager.set_level(1);
        villager.set_experience(0);
    }
}
