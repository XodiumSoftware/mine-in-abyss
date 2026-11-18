//! Alistair, Geomancy Enjoyer, a trader of Orth.
//!
//! The geomancy enjoyer of the **Residential District**.
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
//! | 3 × Orthcoin | 64 × Pointed Dripstone |
//! | 5 × Orthcoin | 64 × Dripstone Block |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Alistair, Geomancy Enjoyer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Alistair;

impl Alistair {
    /// Spawns Alistair, Geomancy Enjoyer in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(84, EntityType::Villager)
            .name(
                TextComponent::text("Alistair, Geomancy Enjoyer").color_rgb(rgb(0x79, 0x55, 0x48)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
