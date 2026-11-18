//! Benner, Also Lord of Currency, a trader of Orth.
//!
//! Boy's companion at the **Golden Bridge**, and the other Lord of Currency.
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
//! | 10 × Mantoyer Horns | 1 × Orthcoin |
//! | 10 × Woodcrier Horn | 1 × Orthcoin |
//! | 14 × Rockwalker Mandible | 1 × Orthcoin |
//! | 16 × Hisoishi Claw | 1 × Orthcoin |
//! | 10 × Buroguro Stinger | 1 × Orthcoin |
//! | 9 × Daikonchu Carapace | 1 × Orthcoin |
//! | 9 × Stinger Horn | 1 × Orthcoin |
//! | 7 × Spittleshrub Root | 1 × Orthcoin |
//! | 6 × Edusan Whisker | 1 × Orthcoin |
//! | 7 × Swarmshocker Jelly | 1 × Orthcoin |
//! | 7 × Tachikatana Scale | 1 × Orthcoin |
//! | 6 × Tachikatana Claw | 1 × Orthcoin |
//! | 10 × Shiratsubi Wing | 1 × Orthcoin |
//! | 10 × Spikewalker Meat | 1 × Orthcoin |

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Benner, Also Lord of Currency.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Benner;

impl Benner {
    /// Spawns Benner, Also Lord of Currency in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(67, EntityType::Villager)
            .name(
                TextComponent::text("Benner, Also Lord of Currency")
                    .color_rgb(rgb(0xF0, 0xC4, 0x38)),
            )
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Librarian);
                }
            })
            .build(world, position)
    }
}
