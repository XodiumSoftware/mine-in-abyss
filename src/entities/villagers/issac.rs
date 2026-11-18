//! Issac, the Intricate Glazer, a trader of Orth.
//!
//! The glazed terracotta trader of the **Giant Tree District**.
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
//! | 1 × Orthcoin | 32 × Gray Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Black Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Red Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Orange Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Yellow Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Lime Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Green Glazed Terracotta |
//! | 1 × Orthcoin | 32 × White Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Light Gray Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Light Blue Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Brown Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Cyan Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Blue Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Purple Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Magenta Glazed Terracotta |
//! | 1 × Orthcoin | 32 × Pink Glazed Terracotta |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Issac, the Intricate Glazer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Issac;

impl Issac {
    /// Spawns Issac, the Intricate Glazer in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(100, EntityType::Villager)
            .name(
                TextComponent::text("Issac, the Intricate Glazer").color_rgb(rgb(0xE5, 0x73, 0x73)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
