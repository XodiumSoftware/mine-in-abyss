//! Robert, Enchanter, a trader of Orth.
//!
//! A book trader of the **Tower District**.
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

/// Robert, Enchanter.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Robert;

impl Robert {
    /// Spawns Robert, Enchanter in `world` at `position`.
    #[expect(dead_code, reason = "no caller places a trader yet")]
    pub(crate) fn entity(world: &World, position: (f64, f64, f64)) -> Entity {
        EntityBuilder::of(93, EntityType::Villager)
            .name(TextComponent::text("Robert, Enchanter").color_rgb(rgb(0x7E, 0x57, 0xC2)))
            .apply(|entity| {
                if let Some(villager) = entity.cast::<Villager>() {
                    villager.set_profession(VillagerProfession::Weaponsmith);
                }
            })
            .build(world, position)
    }
}
