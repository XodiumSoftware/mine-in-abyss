//! Hamilton, Weaponsmith, a trader of Orth.
//!
//! A book trader of the **Tower District**.
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
//! | 13 × Orthcoin | 1 × Looting III |
//! | 11 × Orthcoin | 1 × Sweeping Edge III |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Hamilton, Weaponsmith.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Hamilton;

impl Hamilton {
    /// Spawns Hamilton, Weaponsmith in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(98, EntityType::Villager)
            .name(TextComponent::text("Hamilton, Weaponsmith").color_rgb(rgb(0x35, 0x7A, 0xBD)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Weaponsmith);
                }
            })
            .build(world, position)
    }
}
