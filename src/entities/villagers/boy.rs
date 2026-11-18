//! Boy, the Lord of Currency, and Orth's relic buyer.
//!
//! In the Abyss this is the counter of the **Golden Bridge** in Orth's central district,
//! where a delver turns the things they cut off the creatures below into coin. He and his
//! companion Benner are the **Lords of Currency**: they give one Orthcoin for a bundle of a
//! single trophy, and that coin is what every other trader in the city charges.
//!
//! The offers below are the wiki's, kept as the record to deliver once the plugin API gains a
//! call that sets a villager's offers. Nothing reads them yet: `villager-data` carries only
//! profession, level and experience, so a trade cannot be delivered.
//!
//! # Trades
//!
//! | Give | Receive |
//! | ---- | ------- |
//! | 7 × Orb Piercer Spike | 1 × Orthcoin |
//! | 4 × Orb Piercer Face Fragments | 1 × Orthcoin |
//! | 8 × Splitjaw Scales | 1 × Orthcoin |
//! | 9 × Splitjaw Gunk | 1 × Orthcoin |
//! | 6 × Tesuchi Scales | 1 × Orthcoin |
//! | 8 × Silkfang Eye | 1 × Orthcoin |
//! | 9 × Silkfang Silk | 1 × Orthcoin |
//! | 10 × Cyatoria Feather | 1 × Orthcoin |
//! | 10 × Rohana Tail | 1 × Orthcoin |
//! | 14 × Rohana Powder | 1 × Orthcoin |
//! | 12 × Dosetori Wing | 1 × Orthcoin |
//! | 10 × Dosetori Bone | 1 × Orthcoin |
//! | 8 × Corpse Weeper Eye | 1 × Orthcoin |
//! | 10 × Corpse Weeper Feather | 1 × Orthcoin |
//! | 8 × Hammerbeak Beak | 1 × Orthcoin |
//! | 10 × Hammerbeak Feather | 1 × Orthcoin |
//! | 9 × Onitsuchi Skull | 1 × Orthcoin |
//! | 10 × Onitsuchi Feather | 1 × Orthcoin |
//! | 8 × Kuongatari Abdomen | 1 × Orthcoin |
//! | 8 × Cooked Kuongatari Abdomen | 1 × Orthcoin |
//! | 8 × Kuongatari Juice | 1 × Orthcoin |
//! | 12 × Madokajack Scales | 1 × Orthcoin |
//! | 11 × Madokajack Bone | 1 × Orthcoin |
//! | 10 × Madokajack Poop | 1 × Orthcoin |
//! | 10 × Parasitic Watershrooms | 1 × Orthcoin |
//! | 6 × Chimokami Tail | 1 × Orthcoin |
//! | 6 × Chimokami Fang | 1 × Orthcoin |
//! | 8 × Ashimite Antlers | 1 × Orthcoin |
//! | 9 × Ashimite Hoof | 1 × Orthcoin |
//! | 7 × Hashofu Body | 1 × Orthcoin |
//! | 7 × Hashofu Eye | 1 × Orthcoin |
//! | 7 × Abyssal Snail Gunk | 1 × Orthcoin |
//! | 7 × Abyssal Snail Shell | 1 × Orthcoin |
//! | 6 × Kakatsumuri Shell | 1 × Orthcoin |
//! | 6 × Kagi Tachikiri Pincer | 1 × Orthcoin |
//! | 5 × Titanjaw Pearl | 1 × Orthcoin |
//! | 7 × Titanjaw Scale | 1 × Orthcoin |
//! | 9 × Makihige Tentacle | 1 × Orthcoin |
//! | 9 × Makihige Ink Sac | 1 × Orthcoin |
//! | 10 × Kazura Squid Ink Sac | 1 × Orthcoin |
//! | 5 × Sakuranbo Dust | 1 × Orthcoin |
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

/// Boy, Lord of Currency.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Boy;

impl Boy {
    /// Spawns Boy, Lord of Currency in `world` at `position`.
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(66, EntityType::Villager)
            .name(TextComponent::text("Boy, Lord of Currency").color_rgb(rgb(0xF0, 0xC4, 0x38)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Librarian);
                }
            })
            .build(world, position)
    }
}
