//! `MeatJohn`, Meat Slinger, a trader of Orth.
//!
//! The meat slinger of the **Residential District**.
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

/// `MeatJohn`, Meat Slinger.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct MeatJohn;

impl MeatJohn {
    /// Spawns `MeatJohn`, Meat Slinger in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(86, EntityType::Villager)
            .name(TextComponent::text("John, Meat Slinger").color_rgb(rgb(0xB2, 0x22, 0x22)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Butcher);
                }
            })
            .build(world, position)
    }
}
