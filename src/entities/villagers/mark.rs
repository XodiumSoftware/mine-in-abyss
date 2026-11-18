//! Mark, Weaponsmith, a trader of Orth.
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
//! | 12 × Orthcoin | 1 × Sharpness V |
//! | 8 × Orthcoin | 1 × Smite V |
//! | 10 × Orthcoin | 1 × Fire Aspect II |
//! | 10 × Orthcoin | 1 × Looting III |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Mark, Weaponsmith.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Mark;

impl Mark {
    /// Spawns Mark, Weaponsmith in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(91, EntityType::Villager)
            .name(TextComponent::text("Mark, Weaponsmith").color_rgb(rgb(0xC6, 0x28, 0x28)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Weaponsmith);
                }
            })
            .build(world, position)
    }
}
