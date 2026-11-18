//! Headmaster, The Head Trader, a trader of Orth.
//!
//! The head trader of the **Central Orth District**, selling heads.
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
//! | 12 × Orthcoin | 1 × Furnace Head |
//! | 12 × Orthcoin | 1 × Swirly Crate Head |
//! | 12 × Orthcoin | 1 × Box Head |
//! | 12 × Orthcoin | 1 × Chest Head |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Headmaster, The Head Trader.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Headmaster;

impl Headmaster {
    /// Spawns Headmaster, The Head Trader in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(78, EntityType::Villager)
            .name(
                TextComponent::text("Headmaster, The Head Trader").color_rgb(rgb(0xC0, 0x39, 0x2B)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cartographer);
                }
            })
            .build(world, position)
    }
}
