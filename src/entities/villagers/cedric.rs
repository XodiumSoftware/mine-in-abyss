//! Cedric, Cementer, a trader of Orth.
//!
//! The cementer of the **Central Orth District**, trading concrete powder.
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
//! | 1 × Orthcoin | 64 × White Concrete Powder |
//! | 1 × Orthcoin | 64 × Light Gray Concrete Powder |
//! | 1 × Orthcoin | 64 × Gray Concrete Powder |
//! | 1 × Orthcoin | 64 × Black Concrete Powder |
//! | 1 × Orthcoin | 64 × Red Concrete Powder |
//! | 1 × Orthcoin | 64 × Orange Concrete Powder |
//! | 1 × Orthcoin | 64 × Yellow Concrete Powder |
//! | 1 × Orthcoin | 64 × Lime Concrete Powder |
//! | 1 × Orthcoin | 64 × Green Concrete Powder |
//! | 1 × Orthcoin | 64 × Cyan Concrete Powder |
//! | 1 × Orthcoin | 64 × Light Blue Concrete Powder |
//! | 1 × Orthcoin | 64 × Blue Concrete Powder |
//! | 1 × Orthcoin | 64 × Purple Concrete Powder |
//! | 1 × Orthcoin | 64 × Magenta Concrete Powder |
//! | 1 × Orthcoin | 64 × Pink Concrete Powder |
//! | 1 × Orthcoin | 64 × Brown Concrete Powder |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Cedric, Cementer.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Cedric;

impl Cedric {
    /// Spawns Cedric, Cementer in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(74, EntityType::Villager)
            .name(TextComponent::text("Cedric, Cementer").color_rgb(rgb(0xB0, 0xB0, 0xB0)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Mason);
                }
            })
            .build(world, position)
    }
}
