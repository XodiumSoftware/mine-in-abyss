//! Evril, Void Seeker, a trader of Orth.
//!
//! The void seeker of the **Residential District**.
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

/// Evril, Void Seeker.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Evril;

impl Evril {
    /// Spawns Evril, Void Seeker in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(83, EntityType::Villager)
            .name(TextComponent::text("Evril, Void Seeker").color_rgb(rgb(0x5B, 0x2C, 0x6F)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Cleric);
                }
            })
            .build(world, position)
    }
}
