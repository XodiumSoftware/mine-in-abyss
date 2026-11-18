//! Murdock, Mudboy, a trader of Orth.
//!
//! The mudboy of the **Central Orth District**.
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
//! | 1 × Orthcoin | 64 × Mud |
//! | 1 × Orthcoin | 64 × Muddy Mangrove Roots |
//! | 1 × Orthcoin | 64 × Packed Mud |
//! | 1 × Orthcoin | 64 × Mud Bricks |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Murdock, Mudboy.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Murdock;

impl Murdock {
    /// Spawns Murdock, Mudboy in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(75, EntityType::Villager)
            .name(TextComponent::text("Murdock, Mudboy").color_rgb(rgb(0x6B, 0x4F, 0x38)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
