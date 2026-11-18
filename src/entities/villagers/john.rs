//! John, Wheat Dealer, a trader of Orth.
//!
//! The wheat dealer of the **Central Orth District**.
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
//! | 1 × Orthcoin | 32 × Wheat |
//! | 14 × Orthcoin | 64 × Hay Bale |
//! | 1 × Orthcoin | 32 × Wheat Seeds |
//! | 1 × Orthcoin | 6 × Pumpkin Seeds |
//! | 1 × Orthcoin | 8 × Melon Seeds |
//! | 1 × Orthcoin | 64 × Beetroot Seeds |
//! | 3 × Orthcoin | 64 × Bone Meal |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// John, Wheat Dealer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct John;

impl John {
    /// Spawns John, Wheat Dealer in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(69, EntityType::Villager)
            .name(TextComponent::text("John, Wheat Dealer").color_rgb(rgb(0xD4, 0xA5, 0x17)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Farmer);
                }
            })
            .build(world, position)
    }
}
