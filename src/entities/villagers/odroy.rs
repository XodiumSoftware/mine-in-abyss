//! Odroy, The Apothecary, a trader of Orth.
//!
//! The apothecary of the **Central Orth District**, selling cures and reagents.
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
//! | 1 × Orthcoin | 4 × Magma Cream |
//! | 1 × Orthcoin | 5 × Fuwagi Foot |
//! | 1 × Orthcoin | 10 × Glistering Melon Slice |
//! | 1 × Orthcoin | 2 × Silkfang Eye |
//! | 1 × Orthcoin | 2 × Madokajack Eye |
//! | 1 × Orthcoin | 2 × Billabonk Sac |
//! | 1 × Orthcoin | 3 × Titanjaw Pearl |
//! | 1 × Orthcoin | 2 × Sakuranbo Dust |
//! | 80 × Orthcoin | 1 × Steve Piss |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Odroy, The Apothecary.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Odroy;

impl Odroy {
    /// Spawns Odroy, The Apothecary in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(68, EntityType::Villager)
            .name(TextComponent::text("Odroy, The Apothecary").color_rgb(rgb(0x66, 0xCC, 0x66)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cleric);
                }
            })
            .build(world, position)
    }
}
