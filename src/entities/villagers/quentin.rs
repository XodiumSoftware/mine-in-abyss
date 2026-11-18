//! Keen Weaver, Quentin, a trader of Orth.
//!
//! The banner weaver of the **Giant Tree District**.
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
//! | 4 × Orthcoin | 1 × Banner Pattern (Globe) |
//! | 4 × Orthcoin | 1 × Banner Pattern (Creeper Charge) |
//! | 4 × Orthcoin | 1 × Banner Pattern (Flower Charge) |
//! | 4 × Orthcoin | 1 × Banner Pattern (Snout) |
//! | 4 × Orthcoin | 1 × Banner Pattern (Skull Charge) |
//! | 4 × Orthcoin | 1 × Banner Pattern (Thing) |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Quentin, Keen Weaver.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Quentin;

impl Quentin {
    /// Spawns Quentin, Keen Weaver in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(99, EntityType::Villager)
            .name(TextComponent::text("Quentin, Keen Weaver").color_rgb(rgb(0xE0, 0x91, 0x3D)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Shepherd);
                }
            })
            .build(world, position)
    }
}
