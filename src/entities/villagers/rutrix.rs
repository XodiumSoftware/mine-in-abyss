//! Rutrix, DJ, a trader of Orth.
//!
//! The disc trader of the **Central Orth District**.
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
//! | 10 × Orthcoin | 1 × Pigstep |
//! | 10 × Orthcoin | 1 × 5 |
//! | 10 × Orthcoin | 1 × otherside |
//! | 10 × Orthcoin | 1 × relic |
//! | 10 × Orthcoin | 1 × ward |
//! | 10 × Orthcoin | 1 × strad |
//! | 10 × Orthcoin | 1 × 13 |
//! | 10 × Orthcoin | 1 × stal |
//! | 10 × Orthcoin | 1 × mellohi |
//! | 10 × Orthcoin | 1 × mall |
//! | 10 × Orthcoin | 1 × chirp |
//! | 10 × Orthcoin | 1 × far |
//! | 10 × Orthcoin | 1 × blocks |
//! | 10 × Orthcoin | 1 × cat |
//! | 10 × Orthcoin | 1 × wait |
//! | 10 × Orthcoin | 1 × 11 |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Rutrix, DJ.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Rutrix;

impl Rutrix {
    /// Spawns Rutrix, DJ in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(77, EntityType::Villager)
            .name(TextComponent::text("Rutrix, DJ").color_rgb(rgb(0x9B, 0x30, 0xD8)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cartographer);
                }
            })
            .build(world, position)
    }
}
