//! Norah, The Warped, a trader of Orth.
//!
//! The warped-wood trader of the **Central Orth District**.
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
//! | 3 × Orthcoin | 64 × Warped Stem |
//! | 3 × Orthcoin | 64 × Crimson Stem |
//! | 1 × Orthcoin | 64 × Basalt |
//! | 1 × Orthcoin | 64 × Blackstone |
//! | 4 × Orthcoin | 64 × Gilded Blackstone |
//! | 4 × Orthcoin | 32 × Crying Obsidian |
//! | 4 × Orthcoin | 48 × Shroomlight |
//! | 1 × Orthcoin | 16 × Warped Nylium |
//! | 1 × Orthcoin | 16 × Crimson Nylium |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Norah, The Warped.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Norah;

impl Norah {
    /// Spawns Norah, The Warped in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(71, EntityType::Villager)
            .name(TextComponent::text("Norah, The Warped").color_rgb(rgb(0x2E, 0x8B, 0x8B)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
