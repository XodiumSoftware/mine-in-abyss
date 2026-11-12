//! Boy, the Lord of Currency, and Orth's relic buyer.
//!
//! In the Abyss this is the counter of the **Golden Bridge** in Orth's central district,
//! where a delver turns the things they cut off the creatures below into coin. He and his
//! companion Benner are the **Lords of Currency**: they give one Orthcoin for a bundle of a
//! single trophy, and that coin is what every other trader in the city charges.
//!
//! The wiki's trade table is deliberately **not** written here. The plugin API can read and
//! write a villager's profession, level and experience, but it has no call that sets the
//! offers behind them, so a table in this file would be data nothing could deliver. The trades
//! are the part of [`entity`](Boy::entity) that will grow once the host can be told about them.
//!
//! What it does set is the shape of the trader: a librarian, so its work sound is right, and a
//! name over its head so a player can tell it from a village villager. The trade level and
//! experience are deliberately left alone, so a villager keeps whatever it already had.
//!
//! # A trader is spawned through `EntityBuilder`
//!
//! Unlike a mob, this kind adds no trait of its own: it is a constructor over
//! [`EntityBuilder`](crate::entities::entity_builder::EntityBuilder), the same way
//! [`Orthcoin`](crate::items::orthcoin::Orthcoin) is a constructor over an `ItemStack`. The
//! builder spawns the entity, names it and stamps its id; the downcast inside `apply` is the
//! only part that knows the entity is a villager.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Boy, Lord of Currency: the relic buyer of Orth, built on a librarian.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Boy;

impl Boy {
    /// Spawns Boy in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(1, EntityType::Villager)
            .name(TextComponent::text("Boy, Lord of Currency").color_rgb(rgb(0xF0, 0xC4, 0x38)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Librarian);
                }
            })
            .build(world, position)
    }
}
