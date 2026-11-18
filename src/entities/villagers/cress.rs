//! Cress, Plant Lover, a trader of Orth.
//!
//! The plant lover of the **Residential District**.
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
//! | 1 × Orthcoin | 32 × Moss Block |
//! | 1 × Orthcoin | 64 × Moss Carpet |
//! | 1 × Orthcoin | 64 × Azalea Leaves |
//! | 1 × Orthcoin | 64 × Flowering Azalea Leaves |
//! | 1 × Orthcoin | 32 × Azalea |
//! | 1 × Orthcoin | 32 × Flowering Azalea |
//! | 1 × Orthcoin | 16 × Big Dripleaf |
//! | 1 × Orthcoin | 32 × Small Dripleaf |
//! | 1 × Orthcoin | 32 × Hanging Roots |
//! | 1 × Orthcoin | 32 × Glow Lichen |
//! | 1 × Orthcoin | 16 × Glow Berries |
//! | 1 × Orthcoin | 16 × Large Fern |
//! | 1 × Orthcoin | 16 × Tall Grass |
//! | 1 × Orthcoin | 16 × Pale Hanging Moss |
//! | 1 × Orthcoin | 32 × Pale Moss Block |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Cress, Plant Lover.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cress;

impl Cress {
    /// Spawns Cress, Plant Lover in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(79, EntityType::Villager)
            .name(TextComponent::text("Cress, Plant Lover").color_rgb(rgb(0x4C, 0xAF, 0x50)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Farmer);
                }
            })
            .build(world, position)
    }
}
