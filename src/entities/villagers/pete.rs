//! Pete, Pothead, a trader of Orth.
//!
//! The pothead of the **Residential District**.
//!
//! The wiki's trade table for this counter is deliberately **not** written here: the plugin API
//! has no call that sets a villager's offers, so a table would be data nothing could deliver.
//! What is stated is the shape of the trader - its profession, so the work sound and badge are
//! right, and its name, so a player can tell it from a village villager.

use pumpkin_plugin_api::EntityType;
use pumpkin_plugin_api::Villager;
use pumpkin_plugin_api::VillagerProfession;
use pumpkin_plugin_api::World;
use pumpkin_plugin_api::mobs::EntityCastExt;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::world::Entity;

use crate::entities::entity_builder::EntityBuilder;
use crate::utils::rgb;

/// Pete, Pothead.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Pete;

impl Pete {
    /// Spawns Pete, Pothead in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(87, EntityType::Villager)
            .name(TextComponent::text("Pete, Pothead").color_rgb(rgb(0xD3, 0x7A, 0x34)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cartographer);
                }
            })
            .build(world, position)
    }
}
