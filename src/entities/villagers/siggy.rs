//! Siggy, Strider, a trader of Orth.
//!
//! A book trader of the **Tower District**.
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
//! | 8 × Orthcoin | 1 × Aqua Affinity |
//! | 8 × Orthcoin | 1 × Depth Strider III |
//! | 10 × Orthcoin | 1 × Punch II |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Siggy, Strider.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Siggy;

impl Siggy {
    /// Spawns Siggy, Strider in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(92, EntityType::Villager)
            .name(TextComponent::text("Siggy, Strider").color_rgb(rgb(0x26, 0xA6, 0x9A)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Weaponsmith);
                }
            })
            .build(world, position)
    }
}
