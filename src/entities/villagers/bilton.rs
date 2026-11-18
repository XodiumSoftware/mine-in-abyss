//! Bilton, Protector, a trader of Orth.
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
//! | 12 × Orthcoin | 1 × Protection IV |
//! | 10 × Orthcoin | 1 × Fire Protection IV |
//! | 10 × Orthcoin | 1 × Blast Protection IV |
//! | 10 × Orthcoin | 1 × Projectile Protection IV |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Bilton, Protector.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Bilton;

impl Bilton {
    /// Spawns Bilton, Protector in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(90, EntityType::Villager)
            .name(TextComponent::text("Bilton, Protector").color_rgb(rgb(0x3F, 0x51, 0xB5)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Armorer);
                }
            })
            .build(world, position)
    }
}
