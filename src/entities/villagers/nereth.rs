//! Underworld Merchant, Nereth, a trader of Orth.
//!
//! The underworld merchant of the **Central Orth District**, trading nether goods.
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
//! | 1 × Orthcoin | 32 × Netherrack |
//! | 1 × Orthcoin | 6 × Soul Sand |
//! | 1 × Orthcoin | 6 × Soul Soil |
//! | 1 × Orthcoin | 10 × Nether Quartz Ore |
//! | 1 × Orthcoin | 16 × Magma Block |
//! | 1 × Orthcoin | 7 × Bone Block |
//! | 4 × Orthcoin | 29 × Nether Gold Ore |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Nereth, Underworld Merchant.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Nereth;

impl Nereth {
    /// Spawns Nereth, Underworld Merchant in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(70, EntityType::Villager)
            .name(
                TextComponent::text("Nereth, Underworld Merchant").color_rgb(rgb(0x8B, 0x3A, 0x3A)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
