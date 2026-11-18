//! Larry, Sculk Lord, a trader of Orth.
//!
//! The sculk lord of the **Central Orth District**.
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
//! | 4 × Orthcoin | 64 × Sculk |
//! | 4 × Orthcoin | 64 × Sculk Catalyst |
//! | 4 × Orthcoin | 64 × Sculk Vein |
//! | 8 × Orthcoin | 32 × Sculk Sensor |
//! | 8 × Orthcoin | 4 × Sculk Shrieker |
//! | 4 × Orthcoin | 64 × Ochre Froglight |
//! | 4 × Orthcoin | 64 × Verdant Froglight |
//! | 4 × Orthcoin | 64 × Pearlescent Froglight |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Larry, Sculk Lord.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Larry;

impl Larry {
    /// Spawns Larry, Sculk Lord in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(76, EntityType::Villager)
            .name(TextComponent::text("Larry, Sculk Lord").color_rgb(rgb(0x1C, 0x5B, 0x5B)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
