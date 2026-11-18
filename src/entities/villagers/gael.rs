//! Gael, Glass Master, a trader of Orth.
//!
//! The glass master of the **Central Orth District**.
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
//! | 1 × Orthcoin | 64 × Tinted Glass |
//! | 1 × Orthcoin | 64 × White Stained Glass |
//! | 1 × Orthcoin | 64 × Light Gray Stained Glass |
//! | 1 × Orthcoin | 64 × Gray Stained Glass |
//! | 1 × Orthcoin | 64 × Black Stained Glass |
//! | 1 × Orthcoin | 64 × Cyan Stained Glass |
//! | 1 × Orthcoin | 64 × Purple Stained Glass |
//! | 1 × Orthcoin | 64 × Blue Stained Glass |
//! | 1 × Orthcoin | 64 × Brown Stained Glass |
//! | 1 × Orthcoin | 64 × Green Stained Glass |
//! | 1 × Orthcoin | 64 × Red Stained Glass |
//! | 1 × Orthcoin | 64 × Orange Stained Glass |
//! | 1 × Orthcoin | 64 × Magenta Stained Glass |
//! | 1 × Orthcoin | 64 × Light Blue Stained Glass |
//! | 1 × Orthcoin | 64 × Yellow Stained Glass |
//! | 1 × Orthcoin | 64 × Lime Stained Glass |
//! | 1 × Orthcoin | 64 × Pink Stained Glass |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Gael, Glass Master.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Gael;

impl Gael {
    /// Spawns Gael, Glass Master in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(72, EntityType::Villager)
            .name(TextComponent::text("Gael, Glass Master").color_rgb(rgb(0xA8, 0xD8, 0xE8)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
