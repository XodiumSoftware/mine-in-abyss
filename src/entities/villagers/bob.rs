//! Bob, Handyman, a trader of Orth.
//!
//! The handyman of the **Residential District**.
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
//! | 1 × Orthcoin | 1 × Spyglass |
//! | 1 × Orthcoin | 4 × Compass |
//! | 1 × Orthcoin | 4 × Clock |
//! | 1 × Orthcoin | 1 × Flint and Steel |
//! | 1 × Orthcoin | 1 × Shears |
//! | 1 × Orthcoin | 8 × Lead |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Bob, Handyman.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Bob;

impl Bob {
    /// Spawns Bob, Handyman in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(82, EntityType::Villager)
            .name(TextComponent::text("Bob, Handyman").color_rgb(rgb(0xFF, 0x8C, 0x00)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Toolsmith);
                }
            })
            .build(world, position)
    }
}
