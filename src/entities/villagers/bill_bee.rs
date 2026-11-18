//! `BeeBill`, Bee Keeper, a trader of Orth.
//!
//! The bee keeper of the **Slums District**.
//!
//! What is stated is the shape of the trader - its profession, so the work sound and badge are
//! right, and its name, so a player can tell it from a village villager.
//!
//! # Trades
//!
//! The wiki's offers for this counter, kept here as the record to deliver once the plugin API
//! gains a call that sets a villager's offers. They are documentation only: nothing reads them,
//! and a trade cannot be delivered yet.
//!
//! | Give | Receive |
//! | ---- | ------- |
//! | 5 × Orthcoin | 1 × Bee Nest (2 bees) |
//! | 6 × Orthcoin | 1 × Beehive (2 bees) |
//! | 1 × Orthcoin | 16 × Honey Bottle |
//! | 1 × Orthcoin | 64 × Honeycomb |
//! | 5 × Orthcoin | 64 × Honey Block |
//! | 3 × Orthcoin | 64 × Honeycomb Block |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// `BeeBill`, Bee Keeper.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BeeBill;

impl BeeBill {
    /// Spawns `BeeBill`, Bee Keeper in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(101, EntityType::Villager)
            .name(TextComponent::text("BeeBill, Bee Keeper").color_rgb(rgb(0xE8, 0xB9, 0x23)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Farmer);
                }
            })
            .build(world, position)
    }
}
